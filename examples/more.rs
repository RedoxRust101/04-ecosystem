use anyhow::Result;
use derive_more::{Add, Display, From, Into};

#[derive(PartialEq, Clone, Copy, From, Add, Into, Display)]
struct MyInt(i32);

#[derive(PartialEq, From, Debug)]
struct Point2D {
  x: i32,
  y: i32,
}

/// - PartialEq: compare whether two values are equal, when their internal values are the same type and same value, then return true.
#[derive(Debug, From, Display, Add, PartialEq, Clone, Copy)]
enum MyEnum {
  #[display("int: {_0}")]
  Int(i32),
  Uint(u32),
  #[display("nothing")]
  Nothing,
}

fn main() -> Result<()> {
  let my_int: MyInt = 10.into();
  let v = my_int + 20.into();
  let v1: i32 = v.into();

  println!("my_int: {}, v: {}, v1: {}", my_int, v, v1);

  let p: Point2D = (10, 20).into();
  println!("p: {:?}", p);

  let e: MyEnum = 10i32.into();
  let e1: MyEnum = 10u32.into();
  let e2: MyEnum = MyEnum::Nothing;
  let e4 = (e + 0.into())?;
  println!(
    "e: {}, e1: {:?}, e2: {}, e4: {}, e==e1: {} e==e4: {}, e==MyEnum::Int(30): {}",
    e,
    e1,
    e2,
    e4,
    e == e1,
    e == e4,
    e == MyEnum::Int(30),
  );
  Ok(())
}
