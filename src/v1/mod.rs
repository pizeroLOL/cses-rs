mod time;

use serde_repr::{Deserialize_repr, Serialize_repr};
pub use time::Time;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Subject {
    pub name: String,
    pub simplified_name: Option<String>,
    pub teacher: Option<String>,
    pub room: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class {
    pub subject: String,
    pub start_time: Time,
    pub end_time: Time,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub enum WeekType {
    All,
    Odd,
    Even,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum Weekday {
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
    Sunday = 7,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Schedule {
    pub name: String,
    // TODO: support @rinlit type https://github.com/Class-Widgets/Class-Widgets/blob/main/cses_mgr.py
    pub enable_day: u8,
    pub weeks: WeekType,
    pub classes: Vec<Class>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CsesConfig {
    pub version: u32,
    pub subjects: Vec<Subject>,
    pub schedules: Vec<Schedule>,
}
