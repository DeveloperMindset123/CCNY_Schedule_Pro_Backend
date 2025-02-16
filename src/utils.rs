use std::any::type_name;
use std::hash::{DefaultHasher, Hash, Hasher};

pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

// calculate the hash of a string
pub fn calculate_hash<T: Hash>(t: &T) -> String {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish().to_string()      // convert u64 output to string output
}
// example usage
// fn main() {
//     let x = 21;
//     let y = 2.5;
//     println!("{}", type_of(&y));
//     println!("{}", type_of(x));
// }