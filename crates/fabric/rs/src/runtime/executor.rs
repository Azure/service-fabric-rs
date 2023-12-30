use tokio::runtime::Handle;

// TODO/WIP: The intension here is to be able to plug in the backend runner for
// SF bridges to execute async functions.
// Executor as this cannot be dyn passed due to generic args cannot form vtable.
// To enable this feature the bridges needs to be generic.

// Executor is used by rs to post jobs to execute in the background
pub trait Executor {
    fn spawn(&self, future: impl FnOnce() + std::marker::Send + 'static);
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

impl Executor for DefaultExecutor {
    fn spawn(&self, future: impl FnOnce() + std::marker::Send + 'static) {
        self.rt.spawn(async move { future() });
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
