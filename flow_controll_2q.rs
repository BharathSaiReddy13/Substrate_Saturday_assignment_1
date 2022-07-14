//2q
//if else can be used in assignments
fn main() {
    let n = 5;

    let big_n =
        if n < 10 && n > -10 {
            println!("{}, and is a small number, increase ten-fold", n);

            10 * n
        } else {
            println!("{}, and is a big number, halve the number", n);

            n / 2
        };

    println!("{} -> {}", n, big_n);
} 
