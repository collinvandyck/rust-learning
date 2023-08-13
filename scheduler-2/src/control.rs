use std::collections::HashMap;

use tokio::sync::mpsc;

use crate::{
    command::Command,
    scheduler::{Request, Response, TaskRequest, WaitRequest},
    task::TaskType,
};

pub(crate) struct Control {
    rx: mpsc::Receiver<Request>,
    res_tx: mpsc::Sender<RunResult>,
    res_rx: mpsc::Receiver<RunResult>,
}

impl Control {
    pub(crate) fn new(rx: mpsc::Receiver<Request>) -> Self {
        let (res_tx, res_rx) = mpsc::channel(1024);
        Self { rx, res_tx, res_rx }
    }
    pub(crate) async fn run(&mut self) {
        let mut state = State::new();
        let mut wait: Option<WaitRequest> = None;
        loop {
            if wait.is_some() && state.num_running() == 0 {
                let wr = wait.take().unwrap();
                let _ = wr.tx.send(Response::Accepted);
                println!("Wait completed.");
            }
            tokio::select! {
                Some(res) = self.res_rx.recv() => {
                    match res {
                        RunResult::Finished(typ) => {
                            state.remove(typ);
                        }
                    }
                }
                Some(req) = self.rx.recv() => {
                    match req {
                        Request::Task(TaskRequest{typ, cmd, tx}) => {
                            if !state.try_run(&typ) {
                                let _ = tx.send(Response::Rejected);
                            } else {
                                let res_tx = self.res_tx.clone();
                                tokio::spawn(async move {
                                    let mut runner = Runner::new(typ, cmd, res_tx);
                                    runner.run().await;
                                });
                                let _ = tx.send(Response::Accepted);
                            }
                        }
                        Request::Wait(wr) => {
                            println!("wait");
                            if wait.is_some() {
                                let _ = wr.tx.send(Response::Rejected);
                            } else {
                                wait = Some(wr);
                            }
                        }
                    }
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
    /// Returns the number of currently executing tasks
    fn num_running(&self) -> usize {
        self.running.len()
    }
    fn try_run(&mut self, typ: &TaskType) -> bool {
        if self.running.contains_key(typ) {
            return false;
        }
        self.running.insert(typ.clone(), true);
        true
    }
    fn remove(&mut self, typ: TaskType) {
        self.running.remove(&typ);
    }
}
