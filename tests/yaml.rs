use cses_rs::v1::{
    Class, CsesConfig, NumbericWeekday, Schedule, ShortWeekday, Subject, Time, WeekType, Weekday,
};

fn simple_config() -> CsesConfig {
    CsesConfig {
        version: 1,
        subjects: vec![
            Subject {
                name: "Math".to_string(),
                simplified_name: Some("M".to_string()),
                teacher: Some("Mr. A".to_string()),
                room: Some("101".to_string()),
            },
            Subject {
                name: "English".to_string(),
                simplified_name: Some("E".to_string()),
                teacher: Some("Mr. B".to_string()),
                room: Some("102".to_string()),
            },
            Subject {
                name: "Physics".to_string(),
                simplified_name: Some("P".to_string()),
                teacher: Some("Mr. C".to_string()),
                room: Some("103".to_string()),
            },
            Subject {
                name: "Chemistry".to_string(),
                simplified_name: Some("C".to_string()),
                teacher: Some("Mr. D".to_string()),
                room: Some("104".to_string()),
            },
        ],
        schedules: vec![
            Schedule {
                name: "Monday".to_string(),
                enable_day: Weekday::Short(ShortWeekday::Monday),
                weeks: WeekType::All,
                classes: vec![
                    Class {
                        subject: "Math".to_string(),
                        start_time: Time::new(8, 0, 0),
                        end_time: Time::new(9, 0, 0),
                    },
                    Class {
                        subject: "English".to_string(),
                        start_time: Time::new(9, 0, 0),
                        end_time: Time::new(10, 0, 0),
                    },
                ],
            },
            Schedule {
                name: "Tuesday-Odd".to_string(),
                enable_day: Weekday::Short(ShortWeekday::Tuesday),
                weeks: WeekType::Odd,
                classes: vec![
                    Class {
                        subject: "Physics".to_string(),
                        start_time: Time::new(8, 0, 0),
                        end_time: Time::new(9, 0, 0),
                    },
                    Class {
                        subject: "English".to_string(),
                        start_time: Time::new(9, 0, 0),
                        end_time: Time::new(10, 0, 0),
                    },
                ],
            },
            Schedule {
                name: "Tuesday-Even".to_string(),
                enable_day: Weekday::Short(ShortWeekday::Tuesday),
                weeks: WeekType::Even,
                classes: vec![
                    Class {
                        subject: "Chemistry".to_string(),
                        start_time: Time::new(8, 0, 0),
                        end_time: Time::new(9, 0, 0),
                    },
                    Class {
                        subject: "English".to_string(),
                        start_time: Time::new(9, 0, 0),
                        end_time: Time::new(10, 0, 0),
                    },
                ],
            },
        ],
    }
}

#[test]
fn test_yaml_short_import() {
    let txt = include_str!("short.yaml");
    let import_data: CsesConfig = serde_yaml::from_str(txt).unwrap();
    let rs: CsesConfig = simple_config();
    assert_eq!(import_data, rs);
    let export_data = serde_yaml::to_string(&rs).unwrap();
    assert_eq!(txt, export_data);
}

#[test]
fn test_yaml_numberic_import() {
    let txt = include_str!("numberic.yaml");
    let import_data: CsesConfig = serde_yaml::from_str(txt).unwrap();
    let mut rs = simple_config();
    rs.schedules = simple_config()
        .schedules
        .iter()
        .map(|s| Schedule {
            name: s.name.clone(),
            enable_day: match s.enable_day {
                Weekday::Short(w) => Weekday::Numberic(NumbericWeekday::from_short_weekday(w)),
                Weekday::Numberic(w) => Weekday::Numberic(w),
            },
            weeks: s.weeks,
            classes: s.classes.clone(),
        })
        .collect();
    assert_eq!(import_data, rs);
    let export_data = serde_yaml::to_string(&rs).unwrap();
    assert_eq!(txt, export_data);
}
