use std::fmt::{Display, Formatter};

use chrono::{LocalResult, SecondsFormat, TimeZone, Utc};
use serde::{
    de::{Error, Visitor},
    Deserialize,
    Deserializer,
    Serialize,
};

pub type Timestamp = chrono::DateTime<Utc>;

#[derive(Clone, Debug, Serialize, PartialEq, Eq, PartialOrd)]
pub struct DateTime(#[serde(deserialize_with = "to_datetime")] Timestamp);

impl DateTime {
    pub fn new(ts: Timestamp) -> Self {
        DateTime(ts)
    }

    pub fn into_datetime(self) -> Timestamp {
        self.0
    }

    pub fn now() -> Self {
        Self(Utc::now())
    }
}

impl Default for DateTime {
    fn default() -> Self {
        Self(Timestamp::from_utc(
            chrono::NaiveDateTime::from_timestamp_opt(0, 0).expect("valid parameters were hardcoded"),
            Utc,
        ))
    }
}

impl AsRef<Timestamp> for DateTime {
    fn as_ref(&self) -> &Timestamp {
        &self.0
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_rfc3339_opts(SecondsFormat::Secs, true).as_str())
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        struct DateVisitor;

        impl<'de> Visitor<'de> for DateVisitor {
            type Value = DateTime;

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                f.write_str("an integer or formatted date string")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where E: Error {
                let ts = match Utc.timestamp_opt(v as i64, 0) {
                    LocalResult::None => return Err(E::custom("a valid unix epoch")),
                    LocalResult::Single(t) => t,
                    LocalResult::Ambiguous(t1, _t2) => t1,
                };
                Ok(DateTime(ts))
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where E: Error {
                let ts = s.parse::<Timestamp>().map_err(|e| E::custom(e.to_string()))?;
                Ok(DateTime(ts))
            }
        }

        deserializer.deserialize_any(DateVisitor {})
    }
}

#[cfg(test)]
mod test {
    use chrono::{TimeZone, Utc};

    use crate::models::date_time::DateTime;

    #[test]
    fn deserialize_integers() {
        let ts: DateTime = serde_json::from_str("1659721107").unwrap();
        assert_eq!(
            ts.into_datetime(),
            Utc.with_ymd_and_hms(2022, 8, 5, 17, 38, 27).unwrap()
        );
    }

    #[test]
    fn deserialize_string() {
        let ts: DateTime = serde_json::from_str("\"2022-08-09T17:22:53Z\"").unwrap();
        assert_eq!(
            ts.into_datetime(),
            Utc.with_ymd_and_hms(2022, 8, 9, 17, 22, 53).unwrap()
        );
    }
}
