use crate::{command::Command, control::Control, task::TaskType};
use anyhow::Result;
use std::{future::Future, sync::Arc};
use tokio::sync::{mpsc, oneshot};

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
        let req = Request::new(typ, cmd, tx);
        self.tx.send(req).await?;
        let res = rx.await?;
        Ok(res)
    }
}

pub(crate) struct Request {
    pub(crate) typ: TaskType,
    pub(crate) cmd: Command,
    pub(crate) tx: oneshot::Sender<Response>,
}

impl Request {
    pub(crate) fn new(task_id: TaskType, command: Command, tx: oneshot::Sender<Response>) -> Self {
        Self {
            typ: task_id,
            cmd: command,
            tx,
        }
    }
}

#[derive(Debug)]
pub enum Response {
    Scheduled,
    Rejected,
}
