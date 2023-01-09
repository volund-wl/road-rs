use abi_stable::std_types::*;
use road_macros::*;
use road_types::*;

#[cfg(feature = "hyprland")]
pub mod hypr_road {
    use super::*;
    use hyprland::data::Workspace as HyprWorkspace;
    use hyprland::prelude::*;

    #[derive(Clone)]
    pub struct HyprRoadPlugin;

    impl Plugin for HyprRoadPlugin {
        fn fetch_comp_info(self, dtype: CompInfoTypes) -> CompInfo
        where
            Self: Sized,
        {
            match dtype {
                CompInfoTypes::ActiveWorkspace => {
                    use hyprland::shared::WorkspaceType;
                    let HyprWorkspace {
                        name, id, monitor, ..
                    } = HyprWorkspace::get_active().expect("Error getting active workspace");
                    let name = if !name.is_empty() {
                        RSome(name.into())
                    } else {
                        RNone
                    };
                    let monitor: RString = monitor.into();

                    CompInfo::ActiveWorkspace(Workspace {
                        name,
                        id: match id {
                            WorkspaceType::Unnamed(num) => num,
                            _ => unreachable!(),
                        },
                        monitor_name: monitor,
                    })
                }
                _ => todo!(),
            }
        }
    }
    impl PluginInit for HyprRoadPlugin {
        fn init<'a>() -> PluginWrapper<'a> {
            PluginWrapper {
                plugin: init_plug!(Self),
                xdg_current_desktop: "Hyprland",
            }
        }
    }
}

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
    }
    impl PluginInit for ExamplePlugin {
        fn init<'a>() -> PluginWrapper<'a> {
            PluginWrapper {
                plugin: init_plug!(Self),
                xdg_current_desktop: "Example",
            }
        }
    }
}
