mod time;

use serde_repr::{Deserialize_repr, Serialize_repr};
pub use time::Time;

use serde::{Deserialize, Serialize};

/// 课程
#[derive(Debug, Serialize, Clone, Deserialize, PartialEq)]
pub struct Subject {
    pub name: String,
    pub simplified_name: Option<String>,
    pub teacher: Option<String>,
    pub room: Option<String>,
}

/// 课程安排
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Class {
    pub subject: String,
    pub start_time: Time,
    pub end_time: Time,
}

/// 周启用类型
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub enum WeekType {
    /// 所有周都启用
    All,
    /// 只启用单周
    Odd,
    /// 只启用双周
    Even,
}

/// 星期
#[derive(Debug, PartialEq, Clone, Copy, Serialize_repr, Deserialize_repr)]
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

/// 课程安排启用集，在 weeks 和 enable_day 都匹配的时候启用
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Schedule {
    pub name: String,
    pub enable_day: Weekday,
    pub weeks: WeekType,
    pub classes: Vec<Class>,
}

/// 符合 cses 标准的配置文件数据格式
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CsesConfig {
    pub version: u32,
    pub subjects: Vec<Subject>,
    pub schedules: Vec<Schedule>,
}
