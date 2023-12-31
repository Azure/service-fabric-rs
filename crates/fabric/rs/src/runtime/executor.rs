use std::future::Future;

use log::info;
use tokio::{runtime::Handle, sync::mpsc::channel};

// Executor is used by rs to post jobs to execute in the background
pub trait Executor: Clone {
    // spawns the task to run in background
    fn spawn<F>(&self, future: F)
    where
        F: Future + Send + 'static;

    // run the executor until the ctrl-c os signal
    fn run_until_ctrl_c(&self);
}

#[derive(Clone)]
pub struct DefaultExecutor {
    rt: Handle,
}

impl DefaultExecutor {
    pub fn new(rt: Handle) -> DefaultExecutor {
        DefaultExecutor { rt }
    }
}

// TODO: rt obj needs to be hold somewhere outside of handle
// impl Default for DefaultExecutor {
//     fn default() -> Self {
//         let rt = tokio::runtime::Runtime::new().unwrap();
//         Self {
//             rt: rt.handle().clone(),
//         }
//     }
// }

impl Executor for DefaultExecutor {
    fn spawn<F>(&self, future: F)
    where
        F: Future + Send + 'static,
    {
        self.rt.spawn(async move {
            future.await;
        });
    }

    fn run_until_ctrl_c(&self) {
        info!("DefaultExecutor: setting up ctrl-c event.");
        // set ctrc event
        let (tx, mut rx) = channel(1);
        let handler = move || {
            tx.blocking_send(())
                .expect("Could not send signal on channel.")
        };
        ctrlc::set_handler(handler).expect("Error setting Ctrl-C handler");

        // wait for ctrl-c signal.
        self.rt.block_on(async move {
            info!("DefaultExecutor: Waiting for Ctrl-C...");
            rx.recv().await.expect("Could not receive from channel.");
            info!("DefaultExecutor: Got Ctrl-C! Exiting...");
        });
    }
}

#[cfg(test)]
mod test {
    use super::DefaultExecutor;

    #[test]
    fn test_executor() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let _ex = DefaultExecutor::new(rt.handle().clone());
        // let b_ex: Box<dyn Executor> = Box::new(ex);
    }
}
