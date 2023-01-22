use abi_stable::std_types::*;
use road_macros::*;
use road_types::*;

#[cfg(feature = "sway")]
pub mod sway;

#[cfg(feature = "hyprland")]
pub mod hypr;

//#[cfg(feature = "hyprland")]
//pub use hypr as hypr_road;

pub mod example_plugin {
    use super::*;

    #[derive(Clone)]
    pub struct ExamplePlugin;

    impl Plugin for ExamplePlugin {
        fn fetch_comp_info(self, _dtype: CompInfoTypes) -> CompInfo
        where
            Self: Sized,
        {
            CompInfo::ActiveWorkspace(Workspace {
                name: RSome("example".into()),
                id: 64,
                monitor_name: "eDP-1".into(),
            })
        }
        fn should_run(self) -> bool
        where
            Self: Sized,
        {
            false
        }
        fn info(self) -> PluginInfo
        where
            Self: Sized,
        {
            PluginInfo {
                name: "Example".into(),
                version: "0.1".into(),
                author: "Yavko".into(),
            }
        }
    }
    impl PluginInit for ExamplePlugin {
        fn init<'a>() -> PluginType<'static> {
            init_plug!(Self)
        }
    }
}
