//5q
//function without return return unit -->() 
fn main() {
    let _v: () = ();

    let v = ();
    assert_eq!(v, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}