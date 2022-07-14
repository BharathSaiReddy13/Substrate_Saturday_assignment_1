//3q
//if clone not used at 12th line we cannot use s in line 13
fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    // Convert String to Vec
    let _s = s.clone().into_bytes();
    s
}
