fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }

    println!("The value of y is: {}", y);

    // 足し算
    let sum = 5 + 10;

    // 引き算
    let difference = 95.5 - 4.3;

    // 掛け算
    let product = 4 * 30;

    // 割り算
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // 余り
    let remainder = 43 % 5;

    // 論理型
    let t = true;
    let f: bool = false; // with explicit type annotation

    // 文字
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // 複合型
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    let xx: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = xx.0;
    let six_point_four = xx.1;
    let one = xx.2;

    // 配列
    let a = [1, 2, 3, 4, 5];
    // 要素を変更できないので、月など決まっているものをいれるのがよい
    let months = [
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
}
