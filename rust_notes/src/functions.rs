fn main() {
    let x: i32 = 5;
    print_number(x);
    print_sum(5, 6);

    let x = add_one(x);
    print_number(x);
}

// unlike let, you MUST declare the types of function arguments!!
fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}

fn add_one(x: i32) -> i32 {
    x + 1
    // do not add a semicolon in this case!
}
