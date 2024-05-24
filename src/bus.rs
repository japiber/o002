use tokio::sync::broadcast;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::broadcast::error::{SendError};


pub struct Bus<T> {
    tx: Sender<Box<T>>
}

impl<T: Clone> Bus<T> {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel::<Box<T>>(capacity);
        Self {
            tx,
        }
    }

    pub fn subscribe(&self) -> Receiver<Box<T>> {
        self.tx.subscribe()
    }

    pub fn send(&self, msg: T) -> Result<usize, SendError<Box<T>>> {
        self.tx.send(Box::new(msg))
    }
}

