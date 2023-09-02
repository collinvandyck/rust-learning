use crate::prelude::*;
use crate::Args;

#[derive(Debug)]
pub struct Profile {
    pub sso_session: String,
    pub sso_account_id: String,
    pub sso_role_name: String,
    pub region: String,
}

impl Profile {
    pub fn from_profile(profile: &aws_config::profile::Profile) -> Self {
        let get =
            |n: &str| -> String { profile.get(n).map(str::to_string).unwrap_or("".to_string()) };
        let sso_session = get("sso_session");
        let sso_account_id = get("sso_account_id");
        let sso_role_name = get("sso_role_name");
        let region = get("region");
        Self {
            sso_session,
            sso_account_id,
            sso_role_name,
            region,
        }
    }
    pub fn has_sso(&self) -> bool {
        self.sso_session != "" && self.sso_account_id != "" && self.sso_role_name != ""
    }
}

pub async fn set_field(profile: &str, name: &str, value: &str) -> Result<()> {
    let res = Command::new("aws")
        .arg("--profile")
        .arg(profile)
        .arg("configure")
        .arg("set")
        .arg(name)
        .arg(value)
        .status()
        .await?;
    if !res.success() {
        bail!("Failed to set profile field");
    }
    Ok(())
}

pub async fn ensure(profile_name: &str, region: &str) -> Result<Profile> {
    let profile = try_load_profile(profile_name).await;
    match profile {
        Ok(profile) => {
            if profile.has_sso() {
                return Ok(profile);
            }
        }
        Err(err) => {
            info!(err = ?err, "Failed to load profile {}", profile_name);
            set_field(profile_name, "region", region).await?;
        }
    }
    Command::new("aws")
        .arg("--profile")
        .arg(profile_name)
        .arg("configure")
        .arg("sso")
        .status()
        .await?;
    try_load_profile(profile_name).await.map_err(|err| {
        anyhow!(
            "Failed to load profile {} after configuring: {}",
            profile_name,
            err
        )
    })
}

async fn try_load_profile(profile_name: &str) -> Result<Profile> {
    let fs = Fs::default();
    let env = Env::default();
    let files = ProfileFiles::default();
    let profile_override = None;
    let profile_set = aws_config::profile::load(&fs, &env, &files, profile_override).await?;
    profile_set
        .get_profile(profile_name)
        .map(Profile::from_profile)
        .ok_or(anyhow!("profile not found"))
}
