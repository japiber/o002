use std::time::{Duration, Instant};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RequestMessage<T> {
    id: Uuid,
    request: T,
    created_at: Instant,
}

impl<T: Clone> RequestMessage<T> {
    pub fn new(request: T) -> Self {
        Self {
            id: Uuid::new_v4(),
            request,
            created_at: Instant::now()
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn request(&self) -> T {
        self.request.clone()
    }

    pub fn elapsed(&self) -> Duration {
        self.created_at.elapsed()
    }
}