use anyhow::Result;
use serde::Serialize;
use strum::{
  Display, EnumCount, EnumDiscriminants, EnumIs, EnumIter, EnumString, IntoEnumIterator,
  IntoStaticStr, VariantNames,
};

#[allow(unused)]
#[derive(
  Debug, EnumString, EnumCount, EnumDiscriminants, EnumIter, EnumIs, IntoStaticStr, VariantNames,
)]
enum MyEnum {
  A,
  B(String),
  C,
  D,
}

#[allow(unused)]
#[derive(Display, Debug, Serialize)]
enum Color {
  #[strum(serialize = "redred", to_string = "red")]
  Red,
  Green {
    range: usize,
  },
  Blue(usize),
  Yellow,
  #[strum(to_string = "purple with {sat} saturation")]
  Purple {
    sat: usize,
  },
}

fn main() -> Result<()> {
  println!("MyEnum = {:?}", MyEnum::VARIANTS);
  MyEnum::iter().for_each(|v| println!("{:?}", v));
  println!("total: {:?}", MyEnum::COUNT);

  let my_enum = MyEnum::B("hello".into());
  println!("my_enum is B? {:?}", my_enum.is_b());
  let s: &'static str = my_enum.into();
  println!("my_enum = {:?}", s);

  let red = Color::Red;
  let green = Color::Green { range: 10 };
  let blue = Color::Blue(10);
  let yellow = Color::Yellow;
  let purple = Color::Purple { sat: 20 };
  // print all colors
  println!("red: {}, green: {}, blue: {}, yellow: {}, purple:{}", red, green, blue, yellow, purple);

  let red_str = serde_json::to_string(&red)?;
  println!("red_str: {}", red_str);

  Ok(())
}
