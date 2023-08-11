use std::collections::HashMap;

use tokio::sync::mpsc;

use crate::{
    command::Command,
    scheduler::{Request, Response},
    task::TaskType,
};

pub(crate) struct Control {
    rx: mpsc::Receiver<Request>,
    state: State,
    res_tx: mpsc::Sender<RunResult>,
    res_rx: mpsc::Receiver<RunResult>,
}

impl Control {
    pub(crate) fn new(rx: mpsc::Receiver<Request>) -> Self {
        let (res_tx, res_rx) = mpsc::channel(1024);
        let state = State::new();
        Self {
            rx,
            state,
            res_tx,
            res_rx,
        }
    }
    pub(crate) async fn run(&mut self) {
        loop {
            tokio::select! {
                Some(res) = self.res_rx.recv() => {
                    match res {
                        RunResult::Finished(typ) => {
                            self.state.remove(typ);
                        }
                    }
                }
                Some(Request{typ, cmd, tx}) = self.rx.recv() => {
                    let res_tx = self.res_tx.clone();
                    tokio::spawn(async move {
                        let mut runner = Runner::new(typ, cmd, res_tx);
                        runner.run().await;
                    });
                    let _ = tx.send(Response::Scheduled);
                }
            }
        }
    }
}

struct Runner {
    typ: Option<TaskType>,
    cmd: Command,
    res_tx: Option<mpsc::Sender<RunResult>>,
}

impl Runner {
    fn new(typ: TaskType, cmd: Command, res_tx: mpsc::Sender<RunResult>) -> Self {
        Self {
            typ: Some(typ),
            cmd,
            res_tx: Some(res_tx),
        }
    }
    async fn run(&mut self) {
        self.cmd.run().await;
    }
}

impl Drop for Runner {
    fn drop(&mut self) {
        let tx = self.res_tx.take().unwrap();
        let typ = self.typ.take().unwrap();
        tokio::spawn(async move {
            let _ = tx.send(RunResult::Finished(typ)).await;
        });
    }
}

enum RunResult {
    Finished(TaskType),
}

struct State {
    running: HashMap<TaskType, bool>,
}

impl State {
    fn new() -> Self {
        Self {
            running: HashMap::default(),
        }
    }
    fn try_run(&mut self, typ: TaskType) -> bool {
        if self.running.contains_key(&typ) {
            return false;
        }
        self.running.insert(typ, true);
        true
    }
    fn remove(&mut self, typ: TaskType) {
        self.running.remove(&typ);
    }
}
