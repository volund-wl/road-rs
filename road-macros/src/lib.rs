#[macro_export]
macro_rules! init_plug {
    ($struct:ident) => {{
        use abi_stable::sabi_trait::prelude::*;
        road_types::Plugin_TO::from_value($struct, TD_CanDowncast)
    }};
}
