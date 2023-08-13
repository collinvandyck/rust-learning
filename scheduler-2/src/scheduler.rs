use crate::{command::Command, control::Control, hooks::Hooks, task::TaskType};
use anyhow::Result;
use std::{future::Future, sync::Arc};
use tokio::sync::{mpsc, oneshot};

#[derive(Clone)]
pub struct Scheduler {
    tx: Arc<mpsc::Sender<Request>>,
}

impl Scheduler {
    pub fn new(hooks: Option<Box<dyn Hooks + Send + 'static>>) -> Scheduler {
        let (tx, rx) = mpsc::channel(1024);
        tokio::spawn(async move {
            let mut ctrl = Control::new(rx, hooks);
            ctrl.run().await;
        });
        Self { tx: tx.into() }
    }

    pub fn builder() -> SchedulerBuilder {
        SchedulerBuilder { hooks: None }
    }

    pub async fn wait(&self) -> Result<Response> {
        let (tx, rx) = oneshot::channel();
        let req = WaitRequest { tx };
        let req = Request::Wait(req);
        self.tx.send(req).await?;
        let res = rx.await?;
        Ok(res)
    }

    pub async fn run_task<T: Into<TaskType>, F>(&self, typ: T, f: F) -> Result<Response>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let typ = typ.into();
        let cmd = Command::new(f);
        let (tx, rx) = oneshot::channel();
        let req = TaskRequest::new(typ, cmd, tx);
        let req = Request::Task(req);
        self.tx.send(req).await?;
        let res = rx.await?;
        Ok(res)
    }
}

pub struct SchedulerBuilder {
    hooks: Option<Box<dyn Hooks + Send + 'static>>,
}

impl SchedulerBuilder {
    pub fn hooks(mut self, hooks: impl Hooks + Send + 'static) -> Self {
        self.hooks = Some(Box::new(hooks));
        self
    }

    pub fn build(self) -> Scheduler {
        Scheduler::new(self.hooks)
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
    pub typ: TaskType,
    pub cmd: Command,
    pub tx: oneshot::Sender<Response>,
}

impl TaskRequest {
    pub(crate) fn new(task_id: TaskType, command: Command, tx: oneshot::Sender<Response>) -> Self {
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
