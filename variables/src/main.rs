fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("===== shadowing ======");

    let y = 5;

    let y: i32 = y + 1;

    {
        let y: i32 = y * 2;
        println!("The value of x in the inner scope is: {y}");
    }

    println!("The value of x is: {y}");

    let spaces = "      ";
    let spaces = spaces.len();
    println!("{spaces}");

    // with shadowing you can assign value in same variable with different data types
    // let spaces = "      ";
    // let spaces = spaces.len();

    // while mut variables only can assign value with same data types
    // let mut name = "abi";
    // name = name.len();
}
