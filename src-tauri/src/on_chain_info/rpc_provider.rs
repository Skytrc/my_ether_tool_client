use std::collections::HashMap;
use config::Config;

pub struct Provider {
    https: String,
    websocket: String,
    api_key: String,
}

lazy_static! {
    pub static ref CONFIG: HashMap<String, Provider> = {
        
    };
}