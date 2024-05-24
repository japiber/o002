use std::sync::{Arc, OnceLock};
use serde_json::Value;
use crate::bus::Bus;


mod config;
pub use config::{BusConfig, configure};
pub use config::config as bus_config;

mod bus;
mod message;
pub use message::request::RequestMessage;
pub use message::response::ResponseMessage;

pub mod helper;
pub mod handler;
pub mod dispatch;

pub mod error;

use crate::config::config;

use crate::dispatch::{DispatchCommand, DispatchResponse};
use crate::error::JsonResponseError;

type JsonRequestMessage = RequestMessage<DispatchCommand<Value>>;
type JsonResponseMessage = ResponseMessage<DispatchResponse<Value, JsonResponseError>>;


pub type RequestMessageBus = Bus<JsonRequestMessage>;
pub type ResponseMessageBus = Bus<JsonResponseMessage>;


pub fn request_bus() -> Arc<RequestMessageBus> {
    static REQUEST_BUS: OnceLock<Arc<RequestMessageBus>> = OnceLock::new();
    Arc::clone(REQUEST_BUS.get_or_init(|| {
        Arc::new(RequestMessageBus::new(config().request_capacity))
    }))
}

pub fn response_bus() -> Arc<ResponseMessageBus> {
    static RESPONSE_BUS: OnceLock<Arc<ResponseMessageBus>> = OnceLock::new();
    Arc::clone(RESPONSE_BUS.get_or_init(|| {
        Arc::new(ResponseMessageBus::new(config().response_capacity))
    }))
}

