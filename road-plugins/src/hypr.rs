use super::*;
use async_ffi::{FfiFuture, FutureExt};
use hyprland::data::{Workspace as HyprWorkspace, Workspaces as HyprWorkspaces};
use hyprland::event_listener::{AsyncEventListener, EventListener};
use hyprland::shared::WorkspaceType;
use hyprland::{async_closure, prelude::*};
use road_macros::async_fn;

#[derive(Clone, PluginInit)]
pub struct HyprRoadPlugin;

impl Plugin for HyprRoadPlugin {
    fn listener(self, listener: ListenersObj) -> RResult<(), Error>
    where
        Self: Sized,
    {
        let mut hypr_listener = EventListener::new();
        hypr_listener.add_workspace_change_handler(move |work| {
            let name = match work {
                WorkspaceType::Regular(name) => name,
                WorkspaceType::Special(_) => return,
            };
            let work = HyprWorkspaces::get()
                .expect("Error getting workspaces")
                .find(|item| item.name == name)
                .expect("error finding workspace");
            let work = Workspace {
                id: work.id.into(),
                name: RSome(work.name.into()),
                monitor_name: work.monitor.into(),
            };

            listener.clone().active_work(work);
        });
        hypr_listener
            .start_listener()
            .expect("Error creating hyprland listener");
        ROk(())
    }
    // #[async_fn]
    // async fn listener_async(self, listener: &ListenersAsyncObj)
    // where
    //     Self: Sized,
    // {
    //     let mut hypr_listener = AsyncEventListener::new();
    //     hypr_listener.add_workspace_change_handler(async_closure! {move |work| {
    //         let name = match work {
    //             WorkspaceType::Regular(name) => name,
    //             WorkspaceType::Special(_) => return,
    //         };
    //         let work = HyprWorkspaces::get_async().await
    //             .expect("Error getting workspaces")
    //             .find(|item| item.name == name)
    //             .expect("error finding workspace");
    //         let work = Workspace {
    //             id: work.id.into(),
    //             name: RSome(work.name.into()),
    //             monitor_name: work.monitor.into(),
    //         };
    //
    //         listener.clone().active_work(work);
    //     }});
    //     hypr_listener
    //         .start_listener_async()
    //         .await
    //         .expect("Error creating hyprland listener");
    // }

    fn listener_async(self, listener: &ListenersAsyncObj) -> FfiFuture<()> {
        use async_ffi::{FfiFuture, FutureExt};
        let future = async move {
            let mut hypr_listener = AsyncEventListener::new();
            hypr_listener.add_workspace_change_handler(async_closure! {move |work| {
                let name = match work {
                    WorkspaceType::Regular(name) => name,
                    WorkspaceType::Special(_) => return,
                };
                let work = HyprWorkspaces::get_async().await
                    .expect("Error getting workspaces")
                    .find(|item| item.name == name)
                    .expect("error finding workspace");
                let work = Workspace {
                    id: work.id.into(),
                    name: RSome(work.name.into()),
                    monitor_name: work.monitor.into(),
                };

                listener.clone().active_work(work).await;
            }});
            hypr_listener
                .start_listener_async()
                .await
                .expect("Error creating hyprland listener");
        };
        future.into_ffi()
    }

    fn fetch_comp_info(self, dtype: CompInfoTypes) -> CompInfo
    where
        Self: Sized,
    {
        match dtype {
            CompInfoTypes::ActiveWorkspace => {
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
                    id: id.into(),
                    monitor_name: monitor,
                })
            }
            _ => todo!(),
        }
    }
    #[async_fn]
    async fn fetch_comp_info_async(self, dtype: CompInfoTypes) -> CompInfo {
        match dtype {
            CompInfoTypes::ActiveWorkspace => {
                // use hyprland::shared::WorkspaceType;
                let HyprWorkspace {
                    name, id, monitor, ..
                } = HyprWorkspace::get_active_async()
                    .await
                    .expect("Error getting active workspace");
                let name = if !name.is_empty() {
                    RSome(name.into())
                } else {
                    RNone
                };
                let monitor: RString = monitor.into();

                CompInfo::ActiveWorkspace(Workspace {
                    name,
                    id: id.into(),
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
