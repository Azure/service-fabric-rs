/// safe wrapping for runtime
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
    pub fn create(rt: E) -> ::windows_core::Result<Runtime<E>> {
        let com = create_com_runtime()?;
        Ok(Runtime { com_impl: com, rt })
    }

    pub fn register_stateless_service_factory<F>(
        &self,
        servicetypename: &HSTRING,
        factory: F,
    ) -> windows_core::Result<()>
    where
        F: StatelessServiceFactory,
    {
        let rt_cp = self.rt.clone();
        let bridge: IFabricStatelessServiceFactory =
            StatelessServiceFactoryBridge::create(factory, rt_cp).into();
        unsafe {
            self.com_impl
                .RegisterStatelessServiceFactory(servicetypename, &bridge)
        }
    }

    pub fn register_stateful_service_factory(
        &self,
        servicetypename: &HSTRING,
        factory: impl StatefulServiceFactory,
    ) -> windows_core::Result<()> {
        let rt_cp = self.rt.clone();
        let bridge: IFabricStatefulServiceFactory =
            StatefulServiceFactoryBridge::create(factory, rt_cp).into();
        unsafe {
            self.com_impl
                .RegisterStatefulServiceFactory(servicetypename, &bridge)
        }
    }
}
