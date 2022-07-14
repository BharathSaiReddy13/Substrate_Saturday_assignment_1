//7q
//we cannot borrow s as as mutable more than once
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = r1.clone();

    println!("{}, {}", r1, r2);

    println!("Success!");
}
