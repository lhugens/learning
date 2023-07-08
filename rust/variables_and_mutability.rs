//const X: u32 = 4*5+6;

fn main() {
    //let x = 5;
    let mut x = 5;

    println!("The value of x is: {x}");

    x = 6;
    x += 1;
    x /= 2;
    x *= 5;

    println!("The value of x is: {x}");
    
     constants

    const X: u32 = 4*5+6;

    println!("X = {X}")

     shadowing

    let x = 5;

    println!("x outer scope: x = {x}");

    {
        let x = x * 4;
        println!("x inner scope: x = {x}")
    }


    println!("x outer scope: x = {x}");

    let spaces = "   ";
    let spaces = spaces.len();

    println!("{spaces}")

}
