fn foo(r: String) {
    println!("{r}");
}

fn bar(integer: i32) {
    println!("{integer}");
}

fn main() {

    let s = String::from("Hello");

    println!("{s}");

    let mut s1 = String::from("Hello");

    s1.push_str(", world!");

    println!("{s1} \n{s1}");

    let s2 = String::from("hello");

    let s3 = s2.clone();

    println!("s2 = {s2}, s3 = {s3}");

    let x = 5;
    let y = x;

    println!("{x}, {y}");

    foo(s);

    //println!("{s}"); error
    
    let i: i32 = 3;
    bar(i);

    println!("{i}");

    // Copy is implemented, can be used after being passed to a function
    
}
