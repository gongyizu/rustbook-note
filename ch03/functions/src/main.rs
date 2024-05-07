fn main() {
    println!("Hello, world!");

    another_function();

    another_function2(4);

    print_labeled_measurement(5, 'h');

    // a new scope block created with curly brackets is an expression
    let y = {
        let x = 4;
        // semicolon should not exist here, or it will become a statement
        x + 1
    };

    println!("The value of y is: {y}");

    let x = func_return_value();

    println!("The value of x is: {x}");
}

fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn func_return_value() -> i32 {
    5
}
