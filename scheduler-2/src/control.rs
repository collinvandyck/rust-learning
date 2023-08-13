use std::collections::HashMap;

use tokio::sync::mpsc;

use crate::{
    command::Command,
    hooks,
    scheduler::{Request, Response, TaskRequest, WaitRequest},
    task,
};

/// Control is the main synchronization point for running tasks. It receives requests from the
/// scheduler on a channel and then decides what to do with those requests.
pub(crate) struct Control {
    rx: mpsc::Receiver<Request>,
    res_tx: mpsc::Sender<RunResult>,
    res_rx: mpsc::Receiver<RunResult>,
    hooks: hooks::Wrapped,
}

impl Control {
    pub(crate) fn new(rx: mpsc::Receiver<Request>, hooks: hooks::Wrapped) -> Self {
        let (res_tx, res_rx) = mpsc::channel(1024);
        Self {
            rx,
            res_tx,
            res_rx,
            hooks,
        }
    }
    /// The main loop of the Controller.
    pub(crate) async fn run(&mut self) {
        let mut state = State::new();
        let mut wait: Option<WaitRequest> = None;
        loop {
            // if we are waiting and there are no more tasks running, then complete the wait by
            // transmitting on the channel and replacing the option.
            if wait.is_some() && state.num_running() == 0 {
                let wr = wait.take().unwrap();
                let _ = wr.tx.send(Response::Accepted);
            }
            // After we're done with bookkeeping, enter the select.
            tokio::select! {
                Some(res) = self.res_rx.recv() => {
                    match res {
                        RunResult::Finished(typ) => {
                            state.remove(&typ);
                        }
                    }
                }
                Some(req) = self.rx.recv() => {
                    match req {
                        Request::Task(TaskRequest{typ, cmd, tx}) => {
                            // if we are waiting, that means no more tasks should be scheduled
                            // until the wait is complete.
                            if wait.is_some() {
                                let _ = tx.send(Response::Rejected);
                                continue;
                            }
                            // otherwise, try and run the task if we are able to.
                            if !state.try_run(&typ) {
                                let _ = tx.send(Response::Rejected);
                                continue;
                            }
                            let res_tx = self.res_tx.clone();
                            let task_typ = typ.clone();

                            // if we accepted, invoke the hook if it exists. we will block
                            // the scheduler until the hook is completed so that we can
                            // ensure consistency.
                            if let Some(hook) = self.hooks.as_mut() {
                                let fut = hook.on_task_start(&typ);
                                if let Err(e) = fut.await {
                                    println!("Error in hook: {e:?}");
                                }
                            }
                            // finally, spawn the task and send the accepted response.
                            tokio::spawn(async move {
                                let mut runner = Runner::new(task_typ, cmd, res_tx);
                                runner.run().await;
                            });
                            let _ = tx.send(Response::Accepted);
                        }
                        Request::Wait(wr) => {
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
    typ: Option<task::Type>,
    cmd: Command,
    res_tx: Option<mpsc::Sender<RunResult>>,
}

impl Runner {
    fn new(typ: task::Type, cmd: Command, res_tx: mpsc::Sender<RunResult>) -> Self {
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

/// We implement drop for runner so that it drops `res_tx` when it is dropped. This is used to
/// signal to the controller that the task has finished regardless of task behavior.
impl Drop for Runner {
    fn drop(&mut self) {
        let tx = self.res_tx.take().unwrap();
        let typ = self.typ.take().unwrap();
        tokio::spawn(async move {
            let _ = tx.send(RunResult::Finished(typ)).await;
        });
    }
}

/// This enum is used to communicate the result of a task run back to the controller.
enum RunResult {
    Finished(task::Type),
}

/// State is used to keep track of the currently running tasks.
struct State {
    running: HashMap<task::Type, bool>,
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
    /// Return true if we are allowed to run this task type.
    fn try_run(&mut self, typ: &task::Type) -> bool {
        if self.running.contains_key(typ) {
            return false;
        }
        self.running.insert(typ.clone(), true);
        true
    }
    fn remove(&mut self, typ: &task::Type) {
        self.running.remove(typ);
    }
}
