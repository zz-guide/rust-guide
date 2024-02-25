fn main() {
    // mutable_variable();
    namespace();
}


fn immutable_variable() {
    // 定义一个变量
    let a: i32 = 123;
    println!("a: {a}");

    // a = 22;
    // println!("a: {a}"); // cannot assign twice to immutable variable
}

fn mutable_variable() {
    // 定义一个可变的变量
    let mut b: i32 = 123;
    println!("b: {b}");

    b = 22;
    println!("b: {b}");
}

fn namespace() {
    let x: i32 = 22;
    // println!("x: {x}");

    {
        println!("访问外部x: {x}");
        // 括号内的变量不影响外部同名变量
        let x: i32 = 33;
        println!("访问内部x: {x}");
    }

    // println!("访问外部: {x}");
}