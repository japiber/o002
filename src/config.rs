use std::sync::OnceLock;
use serde::Deserialize;



const DEFAULT_RESPONSE_CAPACITY: usize = 36;
const DEFAULT_REQUEST_CAPACITY: usize = 36;
const DEFAULT_RESPONSE_LAPSE: u64 = 120;
const DEFAULT_REQUEST_LAPSE: u64 = 180;

#[derive(Debug, Clone, Deserialize)]
pub struct BusConfig {
    pub response_capacity: usize,
    pub request_capacity: usize,
    pub response_lapse: u64,
    pub request_lapse: u64,
}

static BUS_CONFIG: OnceLock<BusConfig> = OnceLock::new();

pub fn configure(config: Option<BusConfig>) -> BusConfig {
    BUS_CONFIG.get_or_init(|| {
        config.unwrap_or_else(|| BusConfig {
            response_capacity: DEFAULT_RESPONSE_CAPACITY,
            request_capacity: DEFAULT_REQUEST_CAPACITY,
            response_lapse: DEFAULT_RESPONSE_LAPSE,
            request_lapse: DEFAULT_REQUEST_LAPSE,
        })
    }).clone()
}

pub fn config() -> BusConfig {
    configure(None)
}