use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum NumbericWeekday {
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
    Sunday = 7,
}

impl NumbericWeekday {
    pub const fn from_short_weekday(weekday: ShortWeekday) -> Self {
        match weekday {
            ShortWeekday::Monday => NumbericWeekday::Monday,
            ShortWeekday::Tuesday => NumbericWeekday::Tuesday,
            ShortWeekday::Wednesday => NumbericWeekday::Wednesday,
            ShortWeekday::Thursday => NumbericWeekday::Thursday,
            ShortWeekday::Friday => NumbericWeekday::Friday,
            ShortWeekday::Saturday => NumbericWeekday::Saturday,
            ShortWeekday::Sunday => NumbericWeekday::Sunday,
        }
    }
    pub const fn to_short_weekday(&self) -> ShortWeekday {
        match self {
            NumbericWeekday::Monday => ShortWeekday::Monday,
            NumbericWeekday::Tuesday => ShortWeekday::Tuesday,
            NumbericWeekday::Wednesday => ShortWeekday::Wednesday,
            NumbericWeekday::Thursday => ShortWeekday::Thursday,
            NumbericWeekday::Friday => ShortWeekday::Friday,
            NumbericWeekday::Saturday => ShortWeekday::Saturday,
            NumbericWeekday::Sunday => ShortWeekday::Sunday,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub enum ShortWeekday {
    #[serde(alias = "mon")]
    Monday,
    #[serde(alias = "tue")]
    Tuesday,
    #[serde(alias = "wed")]
    Wednesday,
    #[serde(alias = "thu")]
    Thursday,
    #[serde(alias = "fri")]
    Friday,
    #[serde(alias = "sat")]
    Saturday,
    #[serde(alias = "sun")]
    Sunday,
}
