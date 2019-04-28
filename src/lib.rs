
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}