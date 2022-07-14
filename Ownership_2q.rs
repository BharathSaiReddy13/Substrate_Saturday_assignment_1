//2q
//using take_ownership() s1 moved to s2, so we can't use s1 after  that function call
fn main() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String)->String {
    println!("{}", s);
    s
}
