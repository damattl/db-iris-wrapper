#[macro_use] extern crate log;


mod station_dto;
mod station_fetch;
mod timetable_dto;
mod timetable_fetch;

pub mod fetch {
    pub use crate::timetable_fetch::{*};
    pub use crate::station_fetch::{*};
}

pub mod dto {
    pub use crate::station_dto::{*};
    pub use crate::timetable_dto::{*};
}
