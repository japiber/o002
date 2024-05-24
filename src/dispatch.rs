use std::error::Error;
use std::fmt::Debug;
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum InternalCommand {
    Quit
}

#[derive(Debug, Clone)]
pub enum DispatchCommand<C> where C: Debug + Clone + Send {
    App(Box<C>),
    Internal(Box<InternalCommand>)
}

impl<C> From<InternalCommand> for DispatchCommand<C> where C: Debug + Clone + Send {
    fn from(value: InternalCommand) -> Self {
        Self::Internal(Box::new(value))
    }
}

pub type DispatchResult<R, E> = Result<R, DispatcherError<E>>;

#[derive(Debug, Clone)]
pub enum DispatchResponse<T, E> where E: Debug + Clone + Error {
    App(Box<DispatchResult<T, E>>),
    Internal(InternalCommand)
}

impl<R, E> From<DispatchResult<R, E>> for DispatchResponse<R, E>
    where E: Debug + Clone + Error {
    fn from(value: DispatchResult<R, E>) -> Self {
        Self::App(Box::new(value))
    }
}

impl<R, E> From<InternalCommand> for DispatchResponse<R, E>
    where E: Debug + Clone + Error {
    fn from(value: InternalCommand) -> Self {
        Self::Internal(value)
    }
}

#[derive(Debug, Clone)]
pub enum InternalCommandError {
    Terminate(Option<String>)
}

#[derive(Debug, Clone)]
pub enum DispatcherError<E> where E: Debug + Clone + Error {
    AppError(E),
    InternalError(InternalCommandError),
}

pub enum ResultDispatcher {
    Done(bool),
    Pending,
    Abort
}

#[async_trait]
pub trait CommandDispatcher {
    async fn dispatch(&self, target: Uuid) -> ResultDispatcher;
}
