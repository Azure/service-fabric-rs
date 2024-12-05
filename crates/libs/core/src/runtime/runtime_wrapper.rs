use crate::HSTRING;
/// safe wrapping for runtime
use mssf_com::FabricRuntime::{
    IFabricRuntime, IFabricStatefulServiceFactory, IFabricStatelessServiceFactory,
};

use super::{
    create_com_runtime, executor::Executor, stateful::StatefulServiceFactory,
    stateful_bridge::StatefulServiceFactoryBridge, stateless::StatelessServiceFactory,
    stateless_bridge::StatelessServiceFactoryBridge,
};
pub struct Runtime<E>
where
    E: Executor,
{
    com_impl: IFabricRuntime,
    rt: E,
}

impl<E> Runtime<E>
where
    E: Executor,
{
    pub fn create(rt: E) -> crate::Result<Runtime<E>> {
        let com = create_com_runtime()?;
        Ok(Runtime { com_impl: com, rt })
    }

    pub fn register_stateless_service_factory<F>(
        &self,
        servicetypename: &HSTRING,
        factory: F,
    ) -> crate::Result<()>
    where
        F: StatelessServiceFactory + 'static,
    {
        let rt_cp = self.rt.clone();
        let bridge: IFabricStatelessServiceFactory =
            StatelessServiceFactoryBridge::create(factory, rt_cp).into();
        unsafe {
            self.com_impl
                .RegisterStatelessServiceFactory(servicetypename.as_pcwstr(), &bridge)
        }
    }

    pub fn register_stateful_service_factory(
        &self,
        servicetypename: &HSTRING,
        factory: impl StatefulServiceFactory + 'static,
    ) -> crate::Result<()> {
        let rt_cp = self.rt.clone();
        let bridge: IFabricStatefulServiceFactory =
            StatefulServiceFactoryBridge::create(factory, rt_cp).into();
        unsafe {
            self.com_impl
                .RegisterStatefulServiceFactory(servicetypename.as_pcwstr(), &bridge)
        }
    }
}
