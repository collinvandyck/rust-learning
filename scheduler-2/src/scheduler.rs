use crate::{command::Command, control::Control, task::TaskType};
use anyhow::Result;
use std::{future::Future, sync::Arc};
use tokio::sync::{mpsc, oneshot};

#[derive(Clone)]
pub struct Scheduler {
    tx: Arc<mpsc::Sender<Request>>,
}

impl Scheduler {
    pub fn new() -> Scheduler {
        let (tx, rx) = mpsc::channel(1024);
        tokio::spawn(async move {
            let mut ctrl = Control::new(rx);
            ctrl.run().await;
        });
        Self { tx: tx.into() }
    }

    pub async fn schedule<T: Into<TaskType>, F>(&self, typ: T, f: F) -> Result<Response>
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

pub(crate) enum Request {
    Task(TaskRequest),
    Wait(WaitRequest),
}

/// Instructs the scheduler to wait for all currently running tasks to complete. Any other requests
/// that are received while waiting will be rejected.
pub(crate) struct WaitRequest {}

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
