use crate::{
    command::Command,
    control::Control,
    hooks::{Callback, Hooks},
    rules::Rules,
    task::Type,
};
use anyhow::Result;
use std::{future::Future, sync::Arc};
use tokio::sync::{mpsc, oneshot};

#[derive(Clone)]
pub struct Scheduler {
    tx: Arc<mpsc::Sender<Request>>,
}

impl Scheduler {
    #[must_use]
    pub fn new(hooks: Hooks, rules: Rules) -> Scheduler {
        let (tx, rx) = mpsc::channel(1024);
        tokio::spawn(async move {
            let mut ctrl = Control::new(rx, hooks, rules);
            ctrl.run().await;
        });
        Self { tx: tx.into() }
    }

    #[must_use]
    pub fn builder() -> Builder {
        Builder::new()
    }

    /// Waits for all running tasks to complete. Any tasks that are attempted to be scheduled while
    /// waiting will be rejected. Useful for tests, but not in production.
    ///
    /// # Errors
    ///
    /// Returns an error if the scheduler is already waiting or has been shut down.
    /// Returns an error if the scheduler encountered an error running a hook.
    pub async fn wait(&self) -> Result<Response> {
        let (tx, rx) = oneshot::channel();
        let req = WaitRequest { tx };
        let req = Request::Wait(req);
        self.tx.send(req).await?;
        Ok(rx.await?)
    }

    /// Schedules a task to be run. The response will indicate whether or not the task was accepted
    /// or rejected.
    ///
    /// # Errors
    ///
    /// Returns an error if the scheduler has been shut down. Errors should be propagated up the
    /// stack resulting in program termination.
    pub async fn run_task<T: Into<Type>, F>(&self, typ: T, f: F) -> Result<Response>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let typ = typ.into();
        let cmd = Command::new(f);
        let (tx, rx) = oneshot::channel();
        let req = TaskRequest::new(typ, cmd, tx);
        let req = Request::Task(req);
        self.tx.send(req).await?;
        Ok(rx.await?)
    }
}

pub struct Builder {
    hooks: Hooks,
    rules: Rules,
}

impl Builder {
    fn new() -> Self {
        Self {
            hooks: Hooks::default(),
            rules: Rules::default(),
        }
    }

    #[must_use]
    pub fn rules(mut self, rules: Rules) -> Self {
        self.rules = rules;
        self
    }

    #[must_use]
    pub fn hooks(mut self, hooks: Box<impl Callback + Send + Sync + 'static>) -> Self {
        self.hooks = Hooks(Some(hooks));
        self
    }

    #[must_use]
    pub fn build(self) -> Scheduler {
        Scheduler::new(self.hooks, self.rules)
    }
}

impl From<Box<dyn Callback + Send + Sync + 'static>> for Hooks {
    fn from(value: Box<dyn Callback + Send + Sync + 'static>) -> Self {
        Hooks(Some(value))
    }
}

pub(crate) enum Request {
    Task(TaskRequest),
    Wait(WaitRequest),
}

/// Instructs the scheduler to wait for all currently running tasks to complete. Any other requests
/// that are received while waiting will be rejected.
pub(crate) struct WaitRequest {
    /// The scheduler will transmit on this channel once the wait is done
    /// or if the wait is rejected for some reason.
    pub tx: oneshot::Sender<Response>,
}

/// A request to run a particular command/task.
pub(crate) struct TaskRequest {
    pub typ: Type,
    pub cmd: Command,
    pub tx: oneshot::Sender<Response>,
}

impl TaskRequest {
    pub(crate) fn new(task_id: Type, command: Command, tx: oneshot::Sender<Response>) -> Self {
        Self {
            typ: task_id,
            cmd: command,
            tx,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Response {
    Accepted,
    Rejected,
}
