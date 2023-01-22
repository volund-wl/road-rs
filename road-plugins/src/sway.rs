use super::*;

#[derive(Clone)]
pub struct SwayRoadPlugin;

impl Plugin for SwayRoadPlugin {
    fn fetch_comp_info(self, dtype: CompInfoTypes) -> CompInfo
    where
        Self: Sized,
    {
        let mut connection = swayipc::Connection::new().expect("Error connecting to swayipc");
        match dtype {
            CompInfoTypes::ActiveWorkspace => {
                use swayipc::Workspace as SWorkspace;
                let works = connection
                    .get_workspaces()
                    .expect("Error getting workspaces");
                let mut current: Option<SWorkspace> = None;
                for i in works {
                    if i.focused {
                        current = Some(i)
                    }
                }
                let SWorkspace {
                    name, id, output, ..
                } = current.expect("No focused workspace found");

                CompInfo::ActiveWorkspace(Workspace {
                    name: match name.as_str() {
                        "" => RNone,
                        _ => RSome(name.into()),
                    },
                    id,
                    monitor_name: output.into(),
                })
            }
            _ => todo!(),
        }
    }
    fn should_run(self) -> bool
    where
        Self: Sized,
    {
        std::env::var("XDG_CURRENT_DESKTOP") == Ok("Sway".to_string())
    }
    fn info(self) -> PluginInfo
    where
        Self: Sized,
    {
        PluginInfo {
            name: "HyprSway".into(),
            version: "0.1".into(),
            author: "Yavko".into(),
        }
    }
}
impl PluginInit for SwayRoadPlugin {
    fn init<'a>() -> PluginType<'static> {
        init_plug!(Self)
    }
}
