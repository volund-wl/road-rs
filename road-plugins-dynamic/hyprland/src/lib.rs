use abi_stable::{
    export_root_module,
    prefix_type::PrefixTypeTrait,
    sabi_extern_fn,
    sabi_trait::prelude::TD_Opaque,
    std_types::{ROk, RResult},
};

pub use road_plugins::hypr::*;
use road_types::*;

#[export_root_module]
fn instantiate_root_module() -> PluginRef {
    PluginWrapper(new).leak_into_prefix()
}

#[sabi_extern_fn]
pub fn new() -> RResult<PluginType<'static>, Error> {
    println!("Loaded dynamic HyprRoad");
    ROk(Plugin_TO::from_value(HyprRoadPlugin::init(), TD_Opaque))
}
