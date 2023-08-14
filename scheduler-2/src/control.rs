use std::collections::HashMap;

use crate::{
    command::Command,
    hooks::{self, Callback},
    rules::Rules,
    scheduler::{Request, Response, TaskRequest, WaitRequest},
    task,
};
use tokio::sync::mpsc;

/// Control is the main synchronization point for running tasks. It receives requests from the
/// scheduler on a channel and then decides what to do with those requests.
pub(crate) struct Control {
    rx: mpsc::Receiver<Request>,
    res_tx: mpsc::Sender<RunResult>,
    res_rx: mpsc::Receiver<RunResult>,
    hooks: hooks::Hooks,
    rules: Rules,
    running: HashMap<task::Type, usize>,
}

impl Control {
    pub(crate) fn new(rx: mpsc::Receiver<Request>, hooks: hooks::Hooks, rules: Rules) -> Self {
        let (res_tx, res_rx) = mpsc::channel(1024);
        Self {
            rx,
            res_tx,
            res_rx,
            hooks,
            rules,
            running: HashMap::default(),
        }
    }
    /// The main loop of the Controller.
    pub(crate) async fn run(&mut self) {
        let mut wait: Option<WaitRequest> = None;
        loop {
            // if we are waiting and there are no more tasks running, then complete the wait by
            // transmitting on the channel and replacing the option.
            if wait.is_some() && self.total_running() == 0 {
                let wr = wait.take().unwrap();
                let _ = wr.tx.send(Response::Accepted);
            }
            // After we're done with bookkeeping, enter the select.
            tokio::select! {
                Some(res) = self.res_rx.recv() => {
                    match res {
                        RunResult::Finished(typ) => {
                            self.task_finished(&typ);

                            // invoke the hook letting us know that the task has finished.
                            let hook_res = &self.hooks.on_task_complete(&typ).await;
                            if let Err(e) = hook_res {
                                println!("Error in hook.on_task_complete: {e:?}");
                            }
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
                            if !self.try_run(&typ) {
                                let _ = tx.send(Response::Rejected);
                                continue;
                            }
                            let res_tx = self.res_tx.clone();
                            let task_typ = typ.clone();

                            // if we accepted, invoke the hook if it exists. we will block
                            // the scheduler until the hook is completed so that we can
                            // ensure consistency.
                            let hook_res = &self.hooks.on_task_start(&typ).await;
                            if let Err(e) = hook_res {
                                println!("Error in hook: {e:?}");
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
    /// Returns the total number of running tasks.
    fn total_running(&self) -> usize {
        self.running.values().sum()
    }
    fn task_finished(&mut self, typ: &task::Type) {
        let count = self.running.get_mut(typ).unwrap();
        assert!(
            *count > 0,
            "task count is {} for task type {:?}",
            *count,
            typ
        );
        *count -= 1;
    }
    /// Checks to see whether or not we can run a task of this type. If so, then we mark it as
    /// running and return true. Otherwise, we return false.
    fn try_run(&mut self, typ: &task::Type) -> bool {
        let rule = self.rules.get(typ);
        let count = self.running.get(typ).unwrap_or(&0);
        if count >= &rule.max_running {
            // we can't run any more of this task type.
            return false;
        }
        self.running.insert(typ.clone(), *count + 1);
        true
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
