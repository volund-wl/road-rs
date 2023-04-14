use async_ffi::FfiFuture;
use swayipc::{Event, EventType, Node, WorkspaceChange};

use super::*;

#[derive(Clone, PluginInit)]
pub struct SwayRoadPlugin;

impl Plugin for SwayRoadPlugin {
    fn listener(self, listeners: ListenersObj) -> RResult<(), Error>
    where
        Self: Sized,
    {
        let mut events = swayipc::Connection::new()
            .unwrap()
            .subscribe([EventType::Workspace])
            .unwrap();
        while let Some(event) = events.next().transpose().unwrap() {
            match event {
                Event::Workspace(w) => match w.change {
                    WorkspaceChange::Focus => listeners.clone().active_work({
                        let Node {
                            id, name, output, ..
                        } = w.current.unwrap();
                        Workspace {
                            id,
                            name: RSome(name.unwrap().into()),
                            monitor_name: output.unwrap().into(),
                        }
                    }),
                    _ => todo!(),
                },
                _ => unreachable!(),
            }
        }
        ROk(())
    }
    #[async_fn]
    async fn listener_async(self, listeners: &ListenersAsyncObj)
    where
        Self: Sized,
    {
        let mut events = swayipc::Connection::new()
            .unwrap()
            .subscribe([EventType::Workspace])
            .unwrap();
        while let Some(event) = events.next().transpose().unwrap() {
            match event {
                Event::Workspace(w) => match w.change {
                    WorkspaceChange::Focus => listeners.clone().active_work({
                        let Node {
                            id, name, output, ..
                        } = w.current.unwrap();
                        Workspace {
                            id,
                            name: RSome(name.unwrap().into()),
                            monitor_name: output.unwrap().into(),
                        }
                    }),
                    _ => todo!(),
                },
                _ => unreachable!(),
            };
        }
    }

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
    #[async_fn]
    async fn fetch_comp_info_async(self, dtype: CompInfoTypes) -> CompInfo
    where
        Self: Sized,
    {
        let mut connection = swayipc_async::Connection::new()
            .await
            .expect("Error connecting to swayipc");
        match dtype {
            CompInfoTypes::ActiveWorkspace => {
                use swayipc_async::Workspace as SWorkspace;
                let works = connection
                    .get_workspaces()
                    .await
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
