use std::fmt::Debug;
use std::future::Future;
use serde_json::Value;
use tracing::info;
use uuid::Uuid;
use crate::dispatch::{DispatchResponse, DispatchResult};
use crate::helper::send_response;
use crate::{JsonResponseError, ResponseMessage};


pub async fn request<F, T, R>(from: Uuid, req: R, f: F) -> bool
    where
        F: FnOnce(R) -> T,
        T: Future<Output = DispatchResult<Value, JsonResponseError>> + Send,
        R: Send + Debug,
{
    info!("response_handler request {:?} from: {}", req, from);
    let result = f(req).await;
    let msg = ResponseMessage::new(from, DispatchResponse::from(result));
    send_response(msg)
}

pub async fn request_with_source<F, T, S, R>(from: Uuid, src: S, req: R, f: F) -> bool
    where
        F: FnOnce(S, R) -> T,
        T: Future<Output = DispatchResult<Value, JsonResponseError>> + Send,
        R: Send + Debug,
        S: Send + Debug,
{
    info!("response_handler_with_source {:?} request {:?} from: {}", src, req, from);
    let result = f(src, req).await;
    let msg = ResponseMessage::new(from, DispatchResponse::from(result));
    send_response(msg)
}
