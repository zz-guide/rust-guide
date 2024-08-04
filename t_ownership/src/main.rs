fn main() {
    /*
        所有权和借用

        1. Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
        2. 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
        3. 当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)
    */

    // _t();
    _t2();
}

fn _t() {
    let s1 = String::from("hello");
    let s2 = s1; // 所有权发生转移，后边无法再访问s1

    // println!("{}, world!", s1);
    println!("{}, world!", s2);

    let s3 = s2.clone(); // 克隆,s2仍然有所有权
    println!("{}, world!", s2);
    println!("{}, world!", s3);
}

fn _t2() {
    /*
       不管是变量还是引用都可以通过加mut编程可变的
       1. 同一作用域，特定数据只能有一个可变引用
    */
    let mut x = String::from("hello");
    assert_eq!("hello", x);

    let y = &mut x;

    // assert_eq!("hello", x);
    assert_eq!("hello", *y);

    y.push_str("123");
    println!("{}, world!", y)
}

fn _t3() {
    /*
       不同作用域，可变引用和不可变引用可以同时存在；相同作用于不可同时存在
    */

    // let mut s = String::from("hello");

    // let r1 = &s; // 没问题
    // let r2 = &s; // 没问题
    // let r3 = &mut s; // 大问题

    // println!("{}, {}, and {}", r1, r2, r3);
}
