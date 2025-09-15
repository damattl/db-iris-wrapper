use std::sync::Arc;

use wrapper_core::ports::{MessagePort, StationPort, StatusCodePort, StopPort, TrainPort};

pub struct AppService {
    pub api_base: String,
    pub station_repo: Arc<dyn StationPort>,
    pub message_repo: Arc<dyn MessagePort>,
    pub train_repo: Arc<dyn TrainPort>,
    pub stop_repo: Arc<dyn StopPort>,
    pub status_code_repo: Arc<dyn StatusCodePort>
} // TODO: Read more on static
