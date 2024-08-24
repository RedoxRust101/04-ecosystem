use anyhow::Result;
use chrono::{DateTime, Datelike, Utc};
use derive_builder::Builder;

/// ### #[builder...]
/// - setter(into): set fn auto call .into(), avoiding manual call.
/// - setter(strip_option): to strip the Option<T> wrapper when setting the value.
/// - default: auto set default value, e.g. default = "vec![]".
/// - setter(skip): skip generating a setter method for the field, is not be settable.
/// - setter(each(name = "skill", into)): should generate a separate setter method for each element in the Vec<T>, the name parameter is the additional method name, e.g. .skill(impl Into<String>).
/// - setter(custom) : will not auto generate a setter method for the field, but rather allow the user to define it manually.
#[allow(unused)]
#[derive(Builder, Debug)]
#[builder(build_fn(name = "_priv_build"))]
struct User {
  #[builder(setter(into))]
  name: String,

  #[builder(setter(into, strip_option), default)]
  email: Option<String>,

  #[builder(setter(custom))]
  dob: DateTime<Utc>,

  #[builder(setter(skip))]
  age: u32,

  #[builder(default = "vec![]", setter(each(name = "skill", into)))]
  skills: Vec<String>,
}

fn main() -> Result<()> {
  let user = User::build()
    .name("John")
    .email("john@example.com")
    .skills(vec!["debugging".to_string(), "programming".to_string()])
    .skill("rust")
    .skill("go")
    .dob("2000-01-01T00:00:00Z")
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
    let mut user = self._priv_build()?;
    user.age = (Utc::now().year() - user.dob.year()) as _;
    Ok(user)
  }
  pub fn dob(&mut self, value: &str) -> &mut Self {
    self.dob = DateTime::parse_from_rfc3339(value).map(|dt| dt.with_timezone(&Utc)).ok();
    self
  }
}
