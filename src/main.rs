use rand::Rng;
use std::cmp::Ordering;
use std::io;

const MAX_POINT: u32 = 100_000;

fn main() {
    // guess_number()
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
