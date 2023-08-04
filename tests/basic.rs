// use super_enum::*;

// pub enum Fizz {
//     One,
//     Two,
//     Three
// }

// #[super_enum]
// pub enum MyEnum {
//     None,
//     @aggregate(some::path::Foo, some_mod::Bar, Fizz, Buzz),
//     Other(i32, bool),
//     my_field: u32,
// }

// #[test]
// fn test_super_enum() {
//     let a = MyEnum::Two(567);
//     assert_eq!(a.my_field(), 567);
// }
