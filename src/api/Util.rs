pub mod date_deserializer {
    use chrono::{DateTime, Utc, NaiveDateTime};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use serde::de::Error;
    use anyhow::Result;

    fn time_to_json(t: Option<NaiveDateTime>) -> String {
        if let Some(t) = t {
            return DateTime::<Utc>::from_utc(t, Utc).to_rfc3339();
        }

        String::new()
    }

    pub fn serialize<S: Serializer>(time: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error> {
        time_to_json(time.clone()).serialize(serializer)
    }

    /// Deserialize A Zulu Time into a Computer Friendly Item
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error> {
        let mut time: String = Deserialize::deserialize(deserializer)?;

        // Zulu Time Comes in two forms, one that includes milliseconds to 3 number percision and one without
        // Milliseconds will almost never be needed so if there are any they are removed
        // Example of String that will be adjusted
        // "2018-04-05T18:10:58.836Z" -> "2018-04-05T18:10:58Z"
        if time.len() > 20 {
            time = time[..19].to_string();
            time.push('Z');
        }

        Ok(Some(NaiveDateTime::parse_from_str(&time, "%Y-%m-%dT%H:%M:%SZ").map_err(D::Error::custom)?))
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

    pub fn default() -> Option<NaiveDateTime> {
        None
    }
}

#[macro_use]
pub mod macros {
    /// Create A Hashmap
    #[macro_export]
    macro_rules! map {
        ($($k:expr => $v:expr),+) => {
            {
                let mut map = HashMap::new();
                $(map.insert($k, $v);)+
                map
            }
        }
    }

    /**
    Used primarily in PGCRs to place a struct inside
    of the values structs.

    Make a struct with whatever name is given
    **/
    #[macro_export]
    macro_rules! basic {
        ($($a: ident),+) => {
        $(
            #[derive(Deserialize, Serialize, Clone)]
            pub struct $a {
                pub basic: Basic,
            }
        )+
        };
    }

    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Clone)]
    pub struct Basic {
        pub value: f32,
        pub displayValue: String,
    }

    #[macro_export]
    macro_rules! as_item {
        ($i:item) => {
            $i
        };
    }

    /// Enumize something to add certain constants to the enum
    ///
    /// Allows you to create an enum very similar to one in Java, where you can dictate
    /// what goes in each enum. You can then call get() on any of the values in the enum
    /// to get the value that it stores.
    ///
    /// Very useful within the API because there are a lot of enum types that contain values
    /// such as ActivityIdentifier, ActivityMode and DestinyPlatform.
    #[macro_export]
    macro_rules! enumize {
        ($name: ident, $y: ty => {
                $($na: ident, $lit: expr),*
            }
        )  => {
            $crate::as_item!{
                #[derive(Clone, Copy)]
                pub enum $name {
                    $($na),*,
                }
            }

            impl $name {
                pub fn get_all() -> Vec<$name> {
                    vec![$($name::$na),*,]
                }

                pub fn from(code: $y) -> Option<$name> where $y: PartialEq {
                    for n in $name::get_all() {
                        if n.get() == code {
                            return Some(n);
                        }
                    }

                    None
                }

                pub fn get(&self) -> $y {
                    match self {
                        $($name::$na => $lit),*
                    }
                }
            }

            impl std::cmp::PartialEq for $name {
                fn eq(&self, other: &Self) -> bool {
                    use std::mem::discriminant;
                    discriminant(self) == discriminant(other)
                }
            }
        };
    }

    /*
    impl std::cmp::PartialEq for $name {
                fn eq(&self, other: &Self) -> bool {
                    match (self, other) {
                        $(($na, $na) => true,),*
                        _ => false
                    }
                }
            }
     */

    #[test]
    #[ignore]
    fn enumize_test() {
        enumize!(TestEnum, i32 => {
            f, 3,
            y, 6
        } );

        for t in TestEnum::get_all() {
            println!("{}", t == TestEnum::y);
        }
    }

    #[test]
    #[ignore]
    fn enumize_test_string() {
        enumize!(StringEnum, String => {
            t, "t type".to_string(),
            B, "B type".to_string()
        });

        for t in StringEnum::get_all() {
            println!("{}", t.get());
        }
    }
}

