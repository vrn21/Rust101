fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn sub(a: i32, b: i32) -> i32 {
    a - b
}
fn mult(a: i32, b: i32) -> i32 {
    a * b
}
fn div(a: i32, b: i32) -> i32 {
    a / b
}

fn main() {
    let a = 5;
    let b = 6;
    println!(
        "{} {} {} {}",
        { add(a, b) },
        { sub(a, b) },
        { mult(a, b) },
        { div(a, b) }
    );
}
