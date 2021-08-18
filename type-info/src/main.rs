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
