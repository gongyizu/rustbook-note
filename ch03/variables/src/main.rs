fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // shadowing
    let y = 6;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // shadowing - new variable with a different type
    let space = " ";
    let space = space.len();
    println!("Space is: {space}");

    // 25afebbd5af79b77
}
