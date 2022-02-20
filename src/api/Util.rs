pub mod date_deserializer {
    use chrono::{DateTime, Utc, NaiveDateTime};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use serde::de::Error;
    use anyhow::Result;

    fn time_to_json(t: NaiveDateTime) -> String {
        DateTime::<Utc>::from_utc(t, Utc).to_rfc3339()
    }

    pub fn serialize<S: Serializer>(time: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error> {
        time_to_json(time.clone()).serialize(serializer)
    }

    /// Deserialize A Zulu Time into a Computer Friendly Item
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveDateTime, D::Error> {
        let mut time: String = Deserialize::deserialize(deserializer)?;

        // Zulu Time Comes in two forms, one that includes milliseconds to 3 number percision and one without
        // Milliseconds will almost never be needed so if there are any they are removed
        // Example of String that will be adjusted
        // "2018-04-05T18:10:58.836Z" -> "2018-04-05T18:10:58Z"
        if time.len() > 20 {
            time = time[..19].to_string();
            time.push('Z');
        }

        Ok(NaiveDateTime::parse_from_str(&time, "%Y-%m-%dT%H:%M:%SZ").map_err(D::Error::custom)?)
    }

    pub fn parse_from_string(t: String) -> Result<NaiveDateTime> {
        let mut time = t.clone();

        // Zulu Time Comes in two forms, one that includes milliseconds to 3 number percision and one without
        // Milliseconds will almost never be needed so if there are any they are removed
        // Example of String that will be adjusted
        // "2018-04-05T18:10:58.836Z" -> "2018-04-05T18:10:58Z"
        if time.len() > 20 {
            time = time[..19].to_string();
            time.push('Z');
        }

        Ok(NaiveDateTime::parse_from_str(&time, "%Y-%m-%dT%H:%M:%SZ")?)
    }

    #[test]
    pub fn test_times() {
        assert_eq!("2018-04-05 18:10:58", parse_from_string(String::from("2018-04-05T18:10:58.836Z")).expect("Extra Precise Failed").to_string());
        println!("Extra Precise - Working");
        assert_eq!("2018-04-05 18:10:58", parse_from_string(String::from("2018-04-05T18:10:58Z")).expect("Normal Precision Failed").to_string());
        println!("Normal Precision - Working");
    }
}