use std::any::type_name;

pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

// example usage
// fn main() {
//     let x = 21;
//     let y = 2.5;
//     println!("{}", type_of(&y));
//     println!("{}", type_of(x));
// }