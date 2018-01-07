fn main() {
    println!("Hello, world!");

    // Variables...
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // shadwowing:
    let x = x + 42;
    println!("The value of x is: {}", x);


    println!("Difference: {}", 56.7 / 32.2);


    let condition = true;
    // using 'if' as an expression:
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
