use std::fmt;

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{de, Serializer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct User {
    name: String,
    age: u8,
    // dob: DateTime<Utc>,
    skills: Vec<String>,
}

#[derive(Debug, PartialEq)]
struct ChronoUser {
    name: String,
    age: u8,
    dob: DateTime<Utc>,
    skills: Vec<String>,
}

fn main() -> Result<()> {
    let user = User {
        name: "John Doe".to_string(),
        age: 30,
        // dob: Utc::now(),
        skills: vec!["Rust".to_string(), "Python".to_string()],
    };

    let json = serde_json::to_string(&user)?;
    println!("json: {}", json);

    let user1: User = serde_json::from_str(&json)?;
    println!("user1: {:?}", user1);
    assert_eq!(user, user1);

    let user = ChronoUser {
        name: "John Doe".to_string(),
        age: 30,
        dob: Utc::now(),
        skills: vec!["Rust".to_string(), "Python".to_string()],
    };

    let json = serde_json::to_string(&user)?;
    println!("json: {}", json);

    let user1: ChronoUser = serde_json::from_str(&json)?;
    println!("user1: {:?}", user1);
    assert_eq!(user, user1);
    Ok(())
}

impl Serialize for ChronoUser {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ChronoUser", 4)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("age", &self.age)?;
        state.serialize_field("dob", &self.dob)?;
        state.serialize_field("skills", &self.skills)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for ChronoUser {
    fn deserialize<D>(deserializer: D) -> Result<ChronoUser, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "ChronoUser",
            &["name", "age", "dob", "skills"],
            ChronoUserVisitor,
        )
    }
}

struct ChronoUserVisitor;

impl<'de> Visitor<'de> for ChronoUserVisitor {
    type Value = ChronoUser;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct ChronoUser")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<ChronoUser, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let name = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let age = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
        let dob = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(2, &self))?;
        let skills = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(3, &self))?;

        Ok(ChronoUser {
            name,
            age,
            dob,
            skills,
        })
    }

    fn visit_map<A>(self, map: A) -> Result<ChronoUser, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut name = None;
        let mut age = None;
        let mut dob = None;
        let mut skills = None;

        let mut map = map;
        while let Some(key) = map.next_key()? {
            match key {
                "name" => {
                    if name.is_some() {
                        return Err(de::Error::duplicate_field("name"));
                    }
                    name = Some(map.next_value()?);
                }
                "age" => {
                    if age.is_some() {
                        return Err(de::Error::duplicate_field("age"));
                    }
                    age = Some(map.next_value()?);
                }
                "dob" => {
                    if dob.is_some() {
                        return Err(de::Error::duplicate_field("dob"));
                    }
                    dob = Some(map.next_value()?);
                }
                "skills" => {
                    if skills.is_some() {
                        return Err(de::Error::duplicate_field("skills"));
                    }
                    skills = Some(map.next_value()?);
                }
                _ => {
                    let _: serde::de::IgnoredAny = map.next_value()?;
                }
            }
        }

        let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
        let age = age.ok_or_else(|| de::Error::missing_field("age"))?;
        let dob = dob.ok_or_else(|| de::Error::missing_field("dob"))?;
        let skills = skills.ok_or_else(|| de::Error::missing_field("skills"))?;

        Ok(ChronoUser {
            name,
            age,
            dob,
            skills,
        })
    }
}
