use std::time::{Duration, Instant};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ResponseMessage<T> {
    id: Uuid,
    from: Uuid,
    response: T,
    created_at: Instant,
}

impl<T: Clone> ResponseMessage<T> {
    pub fn new(from: Uuid, res: T) -> Self {
        Self {
            id: Uuid::new_v4(),
            response: res,
            from,
            created_at: Instant::now()
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn from(&self) -> Uuid {
        self.from
    }

    pub fn response(&self) -> T {
        self.response.clone()
    }

    pub fn elapsed(&self) -> Duration {
        self.created_at.elapsed()
    }
}