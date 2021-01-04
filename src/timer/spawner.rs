use std::sync::mpsc::SyncSender;
use std::sync::{Arc, Mutex};
use futures::future::FutureExt;
use std::{future::Future};

use super::task::Task;

#[derive(Clone)]
pub struct Spawner {
    pub task_sender: SyncSender<Arc<Task>>,
}


impl Spawner {
    pub fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }
}
