fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");

    println!("{guess}");

    // SCALAR TYPES

    // integer types

    let a: i8 = -1_3;

    println!("{a}");

    let a: u8 = 1;
    let s: i16 = 1;
    let d: i32 = 1;
    let f: i64 = 1;
    let g: i8 = 1;
    let h: i8 = 1;

    println!("{a} {s} {d} {f} {g} {h}");

    let x = 42;

    print_type_of(&x); // i32

    let y = 42.3;

    print_type_of(&y); // i32

    // booleans

    let t = true;

    let f: bool = false;

    println!("t={t}, f={f}");

    // char

    let c = 'z';
    let z: char = 'c';
    let heart_eyed_cat = '😻';

    println!("{c}, {z}, {heart_eyed_cat}");
    
    let tup: (i32, f64, u8) = (500, 3.5, 1);

    let (q, w, e) = tup;

    let el = tup.0;

    println!("{q}, {w}, {e}, {el}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", a);
}
