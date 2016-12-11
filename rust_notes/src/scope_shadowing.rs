fn main() {
    let x: i32 = 17;
    {
        let y: i32 = 3;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y); // this won't work

    // SHADOWING
    let x: i32 = 8;
    {
        println!("{}", x); // prints 8
        let x = 12;
        println!("{}", x); // prints 12
    }

    println!("{}", x); // prints 8
    let x = 42;
    println!("{}", x); // prints 42

    // note:
    // Shadowing and mutable bindings are similar but are two distinct concepts that can't always be used interchangeably.
    // Shadowing enables us to rebind a name to a value of a different type.
    // It is also possible to change the mutability of a binding.
    // Note that shadowing a name does not alter or destroy the value it was bou d to, and the value will continue to exist until it goes out of scope, even if it is no longer accessible by any means"
    let mut x: i32 = 1;
    x = 7;
    let x = x; // x is now immutable and is bound to 7

    let y = 4;
    let y = "I can also be bound to text!"; // y is now of a different type
}
