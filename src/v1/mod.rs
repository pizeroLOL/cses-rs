mod time;
mod weekday;

pub use time::Time;
pub use weekday::{NumbericWeekday, ShortWeekday};

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

#[derive(Debug, Serialize, Deserialize)]
pub enum Weekday {
    Short(ShortWeekday),
    Numberic(NumbericWeekday),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Schedule {
    pub name: String,
    pub enable_day: Weekday,
    pub weeks: WeekType,
    pub classes: Vec<Class>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CsesConfig {
    pub version: u32,
    pub subjects: Vec<Subject>,
    pub schedules: Vec<Schedule>,
}
