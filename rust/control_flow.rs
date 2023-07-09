fn main() {
    let number = 10;

    if number < 5 {
        println!("less than 5")
    } else {
        println!("greater than 5")
    }

    if number == 10 {
        println!("Number is equal to 10")
    } else if number != 9 {
        println!("Number is different than 9")
    } else {
        println!("Other")
    }

    let number = if number == 10 { 5 } else { 4 };
    println!("{number}");

    //loop {
    //    println!("This will loop forever")
    //}

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result = {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut no = 3;

    while no != 0 {
        println!("{no}");

            no -= 1;
    }

    println!("End");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }

    for b in (1..4).rev() {
        println!("{b}")
    }
}




















