use futures::{future::BoxFuture, task::ArcWake};
use std::sync::mpsc::SyncSender;
use std::sync::{Arc, Mutex};

/// A future that can reschedule itself to be polled by an `Executor`.
pub struct Task {
    /// In-progress future that should be pushed to completion.
    ///
    /// The `Mutex` is not necessary for correctness, since we only have
    /// one thread executing tasks at once. However, Rust isn't smart
    /// enough to know that `future` is only mutated from one thread,
    /// so we need to use the `Mutex` to prove thread-safety. A production
    /// executor would not need this, and could use `UnsafeCell` instead.
    pub future: Mutex<Option<BoxFuture<'static, ()>>>,
    /// Handle to place the task itself back onto the task queue.
    pub task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    // Implement `wake` by sending this task back onto the task channel
    // so that it will be polled again by the executor.
    fn wake_by_ref(arc_self: &Arc<Self>) {
        println!("task wake_by_ref");
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued");
    }
}
