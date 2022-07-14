//1q
// everything struct must be assigned with a value
struct Person {
    name: String,
    age: u8,
    hobby: String
}
fn main() {
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("hobby")
    };

    println!("Success!");
} 
