use futures::task::waker_ref;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::task::Context;

use super::task::Task;

/// Task executor that receives tasks off of a channel and runs them.
pub struct Executor {
    pub ready_queue: Receiver<Arc<Task>>,
}

impl Executor {
    pub fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            println!("recv task");
            // Take the future, and if it has not yet completed (is still Some),
            // poll it in an attempt to complete it.
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                // Create a `LocalWaker` from the task itself
                let context = &mut Context::from_waker(&*waker);
                // `BoxFuture<T>` is a type alias for
                // `Pin<Box<dyn Future<Output = T> + Send + 'static>>`.
                // We can get a `Pin<&mut dyn Future + Send + 'static>`
                // from it by calling the `Pin::as_mut` method.
                if future.as_mut().poll(context).is_pending() {
                    // We're not done processing the future, so put it
                    // back in its task to be run again in the future.
                    println!("putback");
                    *future_slot = Some(future);
                }
            } else {
                println!("None...")
            }
        }
    }
}
