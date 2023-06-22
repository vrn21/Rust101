
fn main() {
    let a = 5;
    let b = 6;
    println!(
        "{:?} {:?} {:?} {:?}",
        { add(a, b) },
        { sub(a, b) },
        { mult(a, b) },
        { div(a, b) }
    );
}
