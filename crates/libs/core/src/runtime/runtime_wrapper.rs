use crate::WString;
/// safe wrapping for runtime
use mssf_com::FabricRuntime::{
    IFabricRuntime2, IFabricStatefulServiceFactory, IFabricStatelessServiceFactory,
};

use super::{
    create_com_runtime, executor::Executor, stateful_bridge::StatefulServiceFactoryBridge,
    stateful_traits::IStatefulServiceFactory, stateless_bridge::StatelessServiceFactoryBridge,
    stateless_traits::IStatelessServiceFactory,
};
pub struct Runtime<E>
where
    E: Executor,
{
    com_impl: IFabricRuntime2,
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

    pub fn register_stateless_service_factory(
        &self,
        servicetypename: &WString,
        factory: Box<dyn IStatelessServiceFactory>,
    ) -> crate::Result<()> {
        let rt_cp = self.rt.clone();
        let bridge: IFabricStatelessServiceFactory =
            StatelessServiceFactoryBridge::create(factory, rt_cp).into();
        unsafe {
            self.com_impl
                .RegisterStatelessServiceFactory(servicetypename.as_pcwstr(), &bridge)
        }
        .map_err(crate::Error::from)
    }

    pub fn register_stateful_service_factory(
        &self,
        servicetypename: &WString,
        factory: Box<dyn IStatefulServiceFactory>,
    ) -> crate::Result<()> {
        let rt_cp = self.rt.clone();
        let bridge: IFabricStatefulServiceFactory =
            StatefulServiceFactoryBridge::create(factory, rt_cp).into();
        unsafe {
            self.com_impl
                .RegisterStatefulServiceFactory(servicetypename.as_pcwstr(), &bridge)
        }
        .map_err(crate::Error::from)
    }
}
