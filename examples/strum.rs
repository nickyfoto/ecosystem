use anyhow::Result;
use serde::Serialize;
use strum::{Display, EnumCount, EnumIs, EnumIter, IntoEnumIterator, IntoStaticStr, VariantNames};

#[allow(unused)]
#[derive(VariantNames, EnumIter, Debug, EnumCount, EnumIs, IntoStaticStr)]
enum MyEnum {
    A,
    B(String),
    C,
}

#[derive(Display, Debug, Serialize)]
enum Color {
    // to_string has higher precedence than serialize
    // serialize has nothing to do with serde serialization
    #[strum(serialize = "redred", to_string = "red")]
    Red,
    #[strum(to_string = "yellow")]
    Yellow,
    Green {
        range: usize,
    },
    #[strum(to_string = "purple with {sat} saturation")]
    Purple {
        sat: usize,
    },
    #[strum(serialize = "blueblue")]
    Blue(usize),
}

fn main() -> Result<()> {
    let a = MyEnum::VARIANTS;
    println!("{:?}", a);
    MyEnum::iter().for_each(|v| println!("{:?}", v));
    println!("total: {:?}", MyEnum::COUNT);

    let my_enum = MyEnum::B("Hello".to_string());
    println!("{:?}, {:?}", my_enum, my_enum.is_b());

    let s: &'static str = my_enum.into();
    println!("{:?}", s);

    let red = Color::Red;
    let yellow = Color::Yellow;
    println!("red Debug: {:?}, Display: {}", red, red);
    println!("yellow Debug: {:?}, Display: {}", yellow, yellow);
    let green = Color::Green { range: 10 };
    println!("green Debug: {:?}, Display: {}", green, green);
    let purple = Color::Purple { sat: 30 };
    println!("purple Debug: {:?}, Display: {}", purple, purple);
    let blue = Color::Blue(20);
    println!("blue Debug: {:?}, Display: {}", blue, blue);

    let blue = serde_json::to_string(&blue)?;
    println!("{}", blue);
    Ok(())
}
