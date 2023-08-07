use super_enum::*;

#[super_enum]
pub enum Other {
    Red,
    Blue,
}

#[super_enum]
#[aggregate(MyEnum, Other)]
pub enum Fizz {
    One,
    Two,
    Three,
}

#[super_enum]
#[fields(my_field: u32)]
#[aggregate(Fizz)]
pub enum MyEnum {
    None,
    Other(i32, bool),
}

#[test]
fn test_mutual_aggregation() {
    let _a = MyEnum::Two;
    let _b = MyEnum::None;
    let _c = MyEnum::Three;
    let _a = Fizz::Two;
    let _b = Fizz::None;
    let _c = Fizz::Three;
    let _d = Fizz::Red;
    let _e = MyEnum::Red;
}
