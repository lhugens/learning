fn function_before() {
    println!("Before main");
}

fn main() {
    function_before();
    println!("In main"); 
    function_after();
    function_with_parameters(2, 5);
    function_with_return_values('c');
    let c: char = function_with_return_values_b();
    println!("{c}")

}

fn function_after() {
    println!("After main");
}

fn function_with_parameters(x: i32, y: i32) {
    println!("With arguments, x={x}, y={y}")
    
}

fn function_with_return_values(c: char) -> char {
    return c
}

fn function_with_return_values_b() -> char {
    'd'
}
