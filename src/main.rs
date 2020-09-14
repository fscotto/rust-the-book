use rand::Rng;
use std::cmp::Ordering;
use std::io;

const MAX_POINT: u32 = 100_000;

fn main() {
    // guess_number()
    println!("0°F = {}°C", fahrenheit_to_celsius(0.0));
    println!("0°C = {}°F", celsius_to_fahrenheit(0.0));
    println!("9th Fibonacci Number = {}", fibonacci_nth_number(9));
}

fn guess_number() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}

fn data_types() {
    let b: u8 = 255;
    println!("byte number is {}", b);
    //println!("byte number wrapping b + 1 = {}", b + 1);

    let f1: f32 = 0.2;
    let f2: f32 = 0.1;
    println!("0.2 + 0.1 = {}", f1 + f2);
    println!("0.2 + 0.1 == 0.3 is {}", (f1 + f2) == 0.3);

    // char type
    println!("{}", 'A');

    // tuple type
    let t: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:?}", t);

    let (_x, y, _z) = t;
    println!("The value of y is: {}", y);
    println!("first tuple value is {}", t.0);

    // array type
    let _a = [1, 2, 3, 4, 5];

    // Array is not allowed to grow or shrink in size.
    // Arrays are useful when you want your data allocated
    // on the stack rather than the heap.
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // array with 5 elements of i32 type
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    // a = [3, 3, 3, 3, 3]
    let _a = [3; 5];
}

fn functions() {
    another_function(5, 6);

    let _x = 5;
    let _y = {
        let x = 3;
        x + 1
    };

    let x = five();
    println!("The value of x is: {}", x);
    println!("The value of x + 1 is: {}", plus_one(x));
    println!("The value of x is: {}", x);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

// implicit return
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1 // or return x + 1;
}

fn control_flow() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Because if is an expression, we can use it on the right side of a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is {}", number);

    let mut count = 0;
    // This loop is forever
    loop {
        println!("again!");

        // guard
        if count == 10 {
            break;
        }
        count += 1;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // while loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");

    // iterate over array's elements
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // more coincise alternatives is for loop (best performance also)
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // loop n times with for loop use Range type
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn fahrenheit_to_celsius(t: f64) -> f64 {
    (t - 32.0) * (5.0 / 9.0)
}

fn celsius_to_fahrenheit(t: f64) -> f64 {
    t * (9.0 / 5.0) + 32.0
}

fn fibonacci_nth_number(n: usize) -> f64 {
    let ratio = 1.6180339887;
    let first_numbers = [0.0, 1.0, 1.0, 2.0, 3.0, 5.0];

    if n < 6 {
        return first_numbers[n];
    }

    let mut t = 5.0;
    let mut f = 5.0;
    while t < (n as f64) {
        f = (f * ratio as f64).round();
        t += 1.0;
    }
    return f;
}

fn borrow1() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it’s okay to still
                   // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
