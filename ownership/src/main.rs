fn main() {
    let s = String::from("hello"); // sがスコープに入る
    takes_ownership(s); // sの値が関数にムーブされ...
    // ここではsはもう有効ではない
    let x = 5; // xがスコープに入る
    makes_copy(x); // xはここでも有効
    //　xも関数にムーブされるが、i32はCopyなので、この後もxは有効

    let s1 = gives_ownership(); // gives_ownershipが呼ばれ、戻り値がs1にムーブされる
    let s2 = String::from("hello"); // s2がスコープに入る

    let s3 = String::from("hello"); // s3がスコープに入る
    let len = calculate_length(&s3); // calculate_lengthが呼ばれ、s3への参照が渡される
    println!("The length of '{}' is {}.", s3, len);
} // ここでxとsはスコープを抜ける。しかし、sはムーブされているので、何も起こらない
// sはここでは有効ではない
fn test() {
    let s = "hello"; // sはここから有効になる
} // このスコープはここまで。sはここでは有効ではない

fn test2() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str()関数は、リテラルをStringに付け加える
    println!("{}", s); // これは`hello, world!`と出力する
}

fn takes_ownership(some_string: String) { // some_stringがスコープに入る
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、dropが呼ばれる

fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける

fn gives_ownership() -> String { // gives_ownership関数が呼ばれる
    let some_string = String::from("hello"); // some_stringがスコープに入る
    some_string // some_stringが返され、呼び出し元の関数にムーブされる
}

// takes_and_gives_backは、Stringを受け取り、Stringを返す
fn takes_and_gives_back(a_string: String) -> String { // a_stringがスコープに入る
    a_string // a_stringが返され、呼び出し元の関数にムーブされる
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// ダングリングポインタ(解放済みのメモリを参照する危険なポインタ)は、Rustではコンパイラが許してくれない
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
