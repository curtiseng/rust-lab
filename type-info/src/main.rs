use serde::de;
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct ConnectorTopics {
    pub name: String,
    pub topics: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Inner {
    topics: Vec<String>,
}

impl<'de> de::Deserialize<'de> for ConnectorTopics {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct ConnectorTopicsVisitor;

        impl<'de> de::Visitor<'de> for ConnectorTopicsVisitor {
            type Value = ConnectorTopics;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ConnectorTopics")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                if let Some(key) = map.next_key()? {
                    let value: Inner = map.next_value()?;
                    if let Some(_) = map.next_key::<&str>()? {
                        Err(de::Error::duplicate_field("name"))
                    } else {
                        Ok(Self::Value {
                            name: key,
                            topics: value.topics,
                        })
                    }
                } else {
                    Err(de::Error::missing_field("name"))
                }
            }
        }

        deserializer.deserialize_map(ConnectorTopicsVisitor {})
    }
}

#[derive(Serialize, Deserialize)]
struct MyStruct {
    #[serde(with = "from::string")]
    biguint: u64,

    #[serde(with = "from::string")]
    bigint: i64,
}

// Serde's 'with' attribute allows you to define a serializer and a deserializer in a module.
// You can implement serialize and deserialize as functions in your code, without using modules,
// and use the `deserialize_with` and `serialize_with` attributes instead.
pub mod from {
    // This is just syntactic sugar, so the attribute looks cool like "from::string".
    // You don't need to embed multiple modules.
    pub mod string {
        use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};

        // This deserializer was originally written with u64 in mind. Then it was made generic by
        // changing u64 to T everywhere and adding boundaries. Same with the serializer.
        pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
        where
            D: Deserializer<'de>,
            T: std::str::FromStr,
            <T as std::str::FromStr>::Err: std::fmt::Display,
        {
            String::deserialize(deserializer)?
                .parse::<T>()
                .map_err(|e| D::Error::custom(format!("{}", e)))
        }

        pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
            T: std::fmt::Display,
        {
            format!("{}", value).serialize(serializer)
        }
    }
}

fn main() {
    let input = r#"{
      "test-name": {
        "topics": [
          "topic1",
          "topic2"
        ]
      }
    }"#;

    let result: ConnectorTopics = serde_json::from_str(input).unwrap();

    let expected = ConnectorTopics {
        name: "test-name".into(),
        topics: vec!["topic1".into(), "topic2".into()],
    };

    assert_eq!(result, expected);
}
