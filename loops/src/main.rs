fn main() {
    // loop {
    //     println!("Hello, world!");
    // }

    let mut counter = 0;
    'counter: loop {
        println!("counter = {}", counter);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counter;
            }
            remaining -= 1;
        }
        counter += 1;
    }
    println!("End of loop");

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // 遅い
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // はやい
    for element in a {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
