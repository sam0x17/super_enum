use super_enum::*;

#[super_enum]
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
fn test_super_enum() {
    let _a = MyEnum::Two;
    let _b = MyEnum::None;
    let _c = MyEnum::Three;
    // assert_eq!(a.my_field(), 567);
}
