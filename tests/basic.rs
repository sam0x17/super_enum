use super_enum::*;

pub enum Fizz {
    One,
    Two,
    Three,
}

#[super_enum]
#[fields(my_field: u32)]
#[aggregate(some::path::Foo, some_mod::Bar, Fizz, Buzz)]
pub enum MyEnum {
    None,
    Other(i32, bool),
}

// #[test]
// fn test_super_enum() {
//     let a = MyEnum::Two(567);
//     assert_eq!(a.my_field(), 567);
// }
