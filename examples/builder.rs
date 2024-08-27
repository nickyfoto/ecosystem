use anyhow::Result;
use chrono::{DateTime, Datelike, Utc};
use derive_builder::Builder;

#[allow(unused)]
#[derive(Debug, Builder)]
#[builder(build_fn(name = "_private_build"))]
struct User {
    #[builder(setter(into))]
    name: String,
    #[builder(default = "\"single\".to_string()", setter(into))]
    marital_status: String,
    #[builder(setter(skip))]
    age: u8,
    #[builder(setter(into, strip_option), default)]
    email: Option<String>, // pass a &str, it will into String and add Some
    #[builder(default = "Vec::new()", setter(each(name = skill, into)))]
    skills: Vec<String>,
    #[builder(setter(custom))]
    dob: DateTime<Utc>,
}

fn main() -> Result<()> {
    let user = User::build()
        .name("Alice")
        .email("hello@world.com")
        .skill("A")
        .skill("B")
        .dob("1990-01-01T00:00:00Z")
        .build()?;
    println!("{:?}", user);
    Ok(())
}

impl User {
    fn build() -> UserBuilder {
        UserBuilder::default()
    }
}

impl UserBuilder {
    fn build(&self) -> Result<User> {
        let mut user = self._private_build()?;
        user.age = (Utc::now().year() - self.dob.unwrap().year()) as _;
        Ok(user)
    }

    fn dob(&mut self, dob: &str) -> &mut Self {
        self.dob = Some(
            DateTime::parse_from_rfc3339(dob)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap(),
        );
        self
    }
}
