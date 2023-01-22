
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
                        WorkspaceType::Unnamed(num) => num.into(),
                        _ => unreachable!(),
                    },
                    monitor_name: monitor,
                })
            }
            _ => todo!(),
        }
    }
    fn should_run(self) -> bool
    where
        Self: Sized,
    {
        std::env::var("XDG_CURRENT_DESKTOP") == Ok("Hyprland".to_string())
    }
    fn info(self) -> PluginInfo
    where
        Self: Sized,
    {
        PluginInfo {
            name: "HyprRoad".into(),
            version: "0.1".into(),
            author: "Yavko".into(),
        }
    }
}
impl PluginInit for HyprRoadPlugin {
    fn init<'a>() -> PluginType<'static> {
        init_plug!(Self)
    }
}
