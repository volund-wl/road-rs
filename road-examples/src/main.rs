use async_ffi::FfiFuture;
use async_std::println as aprintln;
use road_macros::async_fn;
use road_rs::{caller, types::*};

#[derive(Clone, Default)]
pub struct EventListeners;

impl Listeners for EventListeners {
    fn active_work(self, work: Workspace) {
        println!("Workspace changed to: {work:#?}");
    }
}

impl ListenersAsync for EventListeners {
    #[async_fn]
    async fn active_work(self, work: Workspace) {
        caller::data::ActiveWorkspace::get_async().await;
        println!("Workspace changed to: {work:#?}")
    }
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let work = caller::data::ActiveWorkspace::get_async().await;
    aprintln!("{work:#?}").await;
    caller::listeners::ListenersAsync::start(EventListeners::default());
    Ok(())
}
