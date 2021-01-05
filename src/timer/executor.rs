use futures::task::waker_ref;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::task::Context;

use super::task::Task;

pub struct Executor {
    pub ready_queue: Receiver<Arc<Task>>,
}

impl Executor {
    pub fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            println!("recv task");
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
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
