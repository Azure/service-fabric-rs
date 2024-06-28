pub mod FabricClient;
pub mod FabricCommon;
pub mod FabricRuntime;
pub mod FabricTransport;
pub mod FabricTypes;

// hack to make generated code link together
use FabricCommon::*;
use FabricTypes::*;
