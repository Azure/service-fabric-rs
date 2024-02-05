use std::future::Future;

use log::info;
use tokio::{runtime::Handle, sync::mpsc::channel};

// Executor is used by rs to post jobs to execute in the background
// Sync is needed due to we use the executor across await boundary.
pub trait Executor: Clone + Sync + Send + 'static {
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
        let h = self.rt.spawn(async move {
            future.await;
        });

        // Monitor user future.
        // If user task has panic, exit the process.
        // TODO: expose a config to control this behavior.
        // It is observed that if user task panics, sf operation are stuck.
        self.rt.spawn(async move {
            let ok = h.await;
            if ok.is_err() {
                info!(
                    "DefaultExecutor: User spawned future paniced {}",
                    ok.unwrap_err()
                );
                std::process::exit(1);
            }
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
