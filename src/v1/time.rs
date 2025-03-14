use std::{
    fmt::{self, Display, Formatter},
    ops::{Add, Sub},
};

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

/// 时间类
/// 24 * 60 * 60
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Time(i64);

impl Time {
    /// 新建时间
    /// h：小时
    /// m：分钟
    /// s：秒
    /// 当分钟和秒大于 60 会自动进位，小于 60 会自动退位
    pub fn new(h: i64, m: i64, s: i64) -> Self {
        Time(h * 60 * 60 + m * 60 + s)
    }

    pub fn h(&self) -> i64 {
        self.0 / 60 / 60
    }

    pub fn m(&self) -> i64 {
        self.0 / 60 % 60
    }

    pub fn s(&self) -> i64 {
        self.0 % 60
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let neg = if self.0 < 0 { "-" } else { "" };
        write!(
            f,
            "{}{:02}:{:02}:{:02}",
            neg,
            self.h().abs(),
            self.m().abs(),
            self.s().abs()
        )
    }
}

impl Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Time, D::Error>
    where
        D: Deserializer<'de>,
    {
        fn parse_num<'de, D>(it: &str) -> Result<i64, D::Error>
        where
            D: Deserializer<'de>,
        {
            it.parse()
                .map_err(|_| de::Error::invalid_type(de::Unexpected::Str(it), &"an integer"))
        }
        let s = String::deserialize(deserializer)?;
        let is_neg = s.starts_with('-');
        let s = if !is_neg {
            s
        } else {
            s.chars()
                .enumerate()
                .filter(|(i, _)| *i != 0)
                .map(|(_, v)| v)
                .collect::<String>()
        };
        let v: Vec<&str> = s.split(':').collect();
        let len = v.len();
        if len != 3 {
            return Err(de::Error::invalid_length(len, &"time's must be HH:mm:ss"));
        }
        let h = parse_num::<D>(v[0])?;
        let m = parse_num::<D>(v[1])?;
        let s = parse_num::<D>(v[2])?;
        let t = h * 60 * 60 + m * 60 + s;
        let o = if is_neg { Time(-t) } else { Time(t) };
        Ok(o)
    }
}

impl Add for Time {
    fn add(self, rhs: Self) -> Self::Output {
        Time(self.0 + rhs.0)
    }

    type Output = Time;
}

impl Sub for Time {
    type Output = Time;

    fn sub(self, rhs: Self) -> Self::Output {
        Time(self.0 - rhs.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_time() {
        let t = Time::new(1, 2, 3);
        assert_eq!(t.h(), 1);
        assert_eq!(t.m(), 2);
        assert_eq!(t.s(), 3);
        assert_eq!(t.to_string(), "01:02:03");
    }

    #[test]
    fn test_neg_time() {
        let t = Time::new(0, -1, -2);
        assert_eq!(t.h(), 0);
        assert_eq!(t.m(), -1);
        assert_eq!(t.s(), -2);
        assert_eq!(t.to_string(), "-00:01:02")
    }

    #[test]
    fn test_time_serde() {
        let t = Time::new(0, 2, 3);
        let s = serde_json::to_string(&t).unwrap();
        assert_eq!(s, "\"00:02:03\"");
        let t2: Time = serde_json::from_str(&s).unwrap();
        assert_eq!(t, t2);
    }

    #[test]
    fn test_time_serde_neg() {
        let t = Time::new(-1, -2, -3);
        let s = serde_json::to_string(&t).unwrap();
        assert_eq!(s, "\"-01:02:03\"");
        let t2: Time = serde_json::from_str(&s).unwrap();
        assert_eq!(t, t2);
    }

    #[test]
    fn test_time_add() {
        let t1 = Time::new(1, 2, 3);
        let t2 = Time::new(4, 5, 6);
        let t3 = t1 + t2;
        assert_eq!(t3.h(), 5);
        assert_eq!(t3.m(), 7);
        assert_eq!(t3.s(), 9);
    }

    #[test]
    fn test_time_sub() {
        let t1 = Time::new(1, 2, 3);
        let t2 = Time::new(4, 5, 6);
        let t3 = t1 - t2;
        assert_eq!(t3.h(), -3);
        assert_eq!(t3.m(), -3);
        assert_eq!(t3.s(), -3);
    }
}
