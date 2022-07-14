//9q
//1..3 means 1,2 not 1, 2, 3
//getting ascii values of char using as i32
fn main() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}",c as i32);
    }
}
