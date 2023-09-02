mod profile;
mod prelude {
    pub use anyhow::{anyhow, bail, Result};
    pub use aws_config::profile::profile_file::ProfileFiles;
    pub use aws_sdk_sso::config::Region;
    pub use aws_types::os_shim_internal::{Env, Fs};
    pub use chrono::{DateTime, Utc};
    pub use clap::Parser;
    pub use serde::Deserialize;
    pub use tokio::{fs, process::Command};
    pub use tracing::{debug, info, Level};
}

use prelude::*;

#[derive(clap::Parser, Debug)]
pub struct Args {
    #[arg(long, default_value = "us-east-1")]
    aws_region: String,

    #[arg(long, default_value = "dfk")]
    aws_profile: String,

    #[arg(long, env, required = true)]
    sso_start_url: String,

    #[arg(long, env, default_value = "us-west-2")]
    sso_region: String,

    #[arg(long, env, required = false)]
    sso_account_id: Option<String>,

    #[arg(long, env, default_value = "dfk")]
    sso_session_name: String,

    #[arg(long, env, default_value = "dfk")]
    sso_profile: String,
}

#[derive(Debug)]
pub struct SSOProfile {
    pub profile_name: String,
    pub region: String,
    pub sso_account_id: String,
    pub sso_region: String,
    pub sso_role_name: String,
    pub sso_start_url: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SSOToken {
    pub access_token: String,
    pub expires_at: String,
    pub region: String,
    pub start_url: String,
    #[serde(skip)]
    pub valid: bool,
}

impl SSOToken {
    fn valid(&self) -> Result<bool> {
        let expires_at = DateTime::parse_from_rfc3339(&self.expires_at)?.with_timezone(&Utc);
        debug!(expires_at = ?expires_at, "Parsed expires_at");
        let now = Utc::now();
        Ok(now < expires_at)
    }
}

struct OptionalSSOToken(Option<SSOToken>);

impl OptionalSSOToken {
    fn valid(&self) -> Result<bool> {
        self.0.as_ref().map_or(Ok(false), |t| t.valid())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::fmt()
        //.with_span_events(FmtSpan::ENTER)
        .with_target(false)
        .with_level(true)
        .with_max_level(Level::DEBUG)
        .init();
    let args = Args::parse();

    // ensures that the profile exists in the credentials file
    let profile = profile::ensure(&args.aws_profile, &args.aws_region).await?;

    Ok(())
}

async fn start_sso_auth_flow(args: &Args) -> Result<()> {
    Command::new("aws")
        .arg("sso")
        .arg("login")
        .arg("--profile")
        .arg(&args.aws_profile)
        .arg("--region")
        .arg(&args.sso_region)
        .arg("--sso-session")
        .arg(&args.sso_session_name)
        .status()
        .await?;
    Ok(())
}

#[tracing::instrument(skip(args))]
async fn get_sso_token(args: &Args) -> Result<OptionalSSOToken> {
    debug!("Getting SSO Token");
    if let Some(profile) = get_sso_profile(&args).await? {
        let mut token = get_cached_sso_token(&profile).await?;
        if let Some(token) = token.as_mut() {
            token.valid = token.valid()?;
            debug!(valid = token.valid, "Found SSO Token");
        }
        return Ok(OptionalSSOToken(token));
    }
    Ok(OptionalSSOToken(None))
}

#[tracing::instrument(skip(profile))]
async fn get_cached_sso_token(profile: &SSOProfile) -> Result<Option<SSOToken>> {
    let home = home::home_dir().ok_or_else(|| anyhow!("no home dir could be detected"))?;
    let dir = home.join(".aws").join("sso").join("cache");
    if !dir.is_dir() {
        debug!("AWS sso cache dir does not exist");
        return Ok(None);
    }
    let mut files = fs::read_dir(&dir).await?;
    let mut tokens = vec![];
    while let Some(entry) = files.next_entry().await? {
        let path = entry.path();
        let content = fs::read_to_string(&path).await?;
        if let Ok(token) = serde_json::from_str::<SSOToken>(&content) {
            debug!(file = ?path, token_url = token.start_url, profile_url = profile.sso_start_url, "Parsed token");
            if token.start_url == profile.sso_start_url {
                debug!(file = ?path, "Found matching cache file");
                tokens.push(token);
            }
        }
    }
    if tokens.len() > 1 {
        bail!(
            "multiple cached tokens found for profile {}",
            profile.profile_name
        );
    }
    Ok(tokens.get(0).map(Clone::clone))
}
#[tracing::instrument(skip(args))]
async fn get_sso_profile(args: &Args) -> Result<Option<SSOProfile>> {
    let fs = Fs::default();
    let env = Env::default();
    let files = ProfileFiles::default();
    let profile_override = None;
    let profile_set = aws_config::profile::load(&fs, &env, &files, profile_override).await?;
    info!(name = args.sso_profile, "Looking for profile");
    let res = match profile_set.get_profile(&args.sso_profile) {
        Some(profile) => {
            let get = |n: &str| -> Result<String> {
                profile
                    .get(n)
                    .map(str::to_string)
                    .ok_or(anyhow!("profile must have {n} set."))
            };
            Some(SSOProfile {
                profile_name: args.sso_profile.clone(),
                region: get("region")?,
                sso_account_id: get("sso_account_id")?,
                sso_region: get("sso_region").unwrap_or(args.sso_region.clone()),
                sso_role_name: get("sso_role_name")?,
                sso_start_url: get("sso_start_url").unwrap_or(args.sso_start_url.clone()),
            })
        }
        None => None,
    };
    Ok(res)
}

#[tracing::instrument(skip(args))]
async fn test_identity(args: &Args) -> Result<()> {
    let sts_client = sts_client(&args).await;
    let identity = sts_client.get_caller_identity().send().await?;
    println!("{identity:?}");
    Ok(())
}

#[tracing::instrument(skip(args))]
async fn sts_client(args: &Args) -> aws_sdk_sts::Client {
    let region = Region::new(args.aws_region.clone());
    let config = aws_config::from_env().region(region).load().await;
    let client = aws_sdk_sts::Client::new(&config);
    client
}

#[tracing::instrument(skip(args))]
async fn sso_client(args: &Args) -> aws_sdk_sso::Client {
    let region = Region::new(args.sso_region.clone());
    let config = aws_config::from_env().region(region).load().await;
    let client = aws_sdk_sso::Client::new(&config);
    client
}
