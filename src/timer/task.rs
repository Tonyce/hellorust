use futures::{future::{BoxFuture}, task::ArcWake};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{SyncSender};
pub struct Task {
    pub future: Mutex<Option<BoxFuture<'static, ()>>>,
    pub task_sender: SyncSender<Arc<Task>>
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("too many tasks queued");
    }
}
