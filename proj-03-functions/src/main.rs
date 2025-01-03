fn main() {
    another_function(5, 'm');

    let y = {
        let x = 6;
        x + 1
    };
    println!("Value of y is {y}");

    let z = five();
    println!("Value of z is {z}");

    let foo = add_five(5);
    println!("Value of foo is {foo}");
}

fn add_five(x: i32) -> i32 {
    x + 5
}

fn another_function(x: i32, unit: char) {
    println!("The value of x is: {x}{unit}");
}

fn five() -> i32 {
    5
}

