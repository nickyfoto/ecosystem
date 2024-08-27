use derive_more::{derive::From, Add, Display, Into};

#[derive(Into, From, Display, Add)]
struct MyInt(i32);

#[derive(Debug, From, Display)]
enum MyEnum {
    #[display("MyEnum::Int({_0})")]
    Int(i32),
    Uint(u32),
    #[display("nothing")]
    Nothing,
}

fn main() {
    let my_int: MyInt = 10.into();

    println!("my_int {}", my_int);
    let v = my_int + 20.into();
    println!("v: {}", v);
    let v1: i32 = v.into();
    println!("v1: {}", v1);

    let e = MyEnum::Int(10);
    println!("e: {:?}", e);
    let e: MyEnum = 20.into();
    println!("e: Debug:{:?}, Display: {}", e, e);
    let e1: MyEnum = 20u32.into();
    println!("e1: Debug:{:?}, Display: {}", e1, e1);
    let e2 = MyEnum::Nothing;
    println!("e2: Debug:{:?}, Display: {}", e2, e2);
}
