use std::io;

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

    // create tuple
    let tup1: (i32, f64, u8) = (500, 6.4, 1);

    // destructure
    let tup2 = (500, 6.4, 1);
    let (x, y, z) = tup2;
    println!("The value of y is: {y}, x and z: {x}, {z}");

    // access tuple by indices
    let five_hundred = tup1.0;
    let six_point_four = tup1.1;
    println!("Five hundred is: {five_hundred}, six point four is: {six_point_four}");

    // array
    let a1 = [1, 2, 3, 4, 5];
    let a2: [i32; 5] = [1, 2, 3, 4, 5];
    let a3 = [3; 5]; // the same as: let a3 = [3, 3, 3, 3, 3];

    println!("a1[1] = {}, a2[0] = {}, a3[3] = {}", a1[1], a2[0], a3[3]);
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a1[index];
    println!("The value of the element at index {index} is: {element}");
}
