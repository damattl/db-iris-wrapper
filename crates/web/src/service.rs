use std::sync::Arc;

use wrapper_core::ports::{MessagePort, StationPort, StopPort, TrainPort};

pub struct AppService {
    pub station_repo: Arc<dyn StationPort>,
    pub message_repo: Arc<dyn MessagePort<'static>>,
    pub train_repo: Arc<dyn TrainPort<'static>>,
    pub stop_repo: Arc<dyn StopPort<'static>>
} // TODO: Read more on static
