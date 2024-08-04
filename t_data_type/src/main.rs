fn main() {
    // _t_integer();
    // _t_binary();
    // _t_float();
    // _t_nan();
    // _t_operation();
    // _t_char();
    _t_bool();
}

fn _t_integer() {
    let i1: i8 = -128; // literal out of range for `i8` -127~127 , 2的7次方
    println!("i8: {}", i1);

    let i1: u8 = 255;
    println!("u8: {}", i1);

    let i1: i16 = 200;
    println!("i16: {}", i1);

    let i1: u16 = 200;
    println!("u16: {}", i1);

    let i1: i32 = 200;
    println!("i32: {}", i1);

    let i1: u32 = 200;
    println!("u32: {}", i1);

    let i1: i64 = 200;
    println!("i64: {}", i1);

    let i1: u64 = 200;
    println!("u64: {}", i1);

    let i1: i128 = 200;
    println!("i128: {}", i1);

    let i1: u128 = 200;
    println!("u128: {}", i1);

    /*
       视架构而定，32位或者64位
    */
    let i1: isize = 200;
    println!("isize: {}", i1);

    let i1: usize = 200;
    println!("usize: {}", i1);

    let i1 = 345;
    println!("默认类型是32: {}", i1);
}

fn _t_binary() {
    /*
        整形字面量
    */

    let i1 = 98_222;
    println!("10进制: {}", i1);

    let i1 = 0xff;
    println!("16进制: {}", i1);

    let i1 = 0o77;
    println!("8进制: {}", i1);

    let i1 = b'A';
    println!("字节 : {}", i1);
}

fn _t_float() {
    /*
        浮点数
    */

    let f1: f32 = 0.3; // f32
    println!("f32: {}", f1);

    let f1 = 0.88888888888888888888; // f64
    println!("f64: {}", f1);
}

fn _t_nan() {
    /*
        对于数学上未定义的结果，例如对负数取平方根 -42.1.sqrt() ，会产生一个特殊的结果：Rust 的浮点数类型使用 NaN (not a number)来处理这些情况。
        NaN不能比较
    */

    let x = (-42.0_f32).sqrt();
    println!("NaN: {}", x);
    if x.is_nan() {
        println!("未定义的数学行为")
    }

    assert_eq!(x, x);
}

fn _t_operation() {
    /*
      运算符
    */
    // 加法
    let sum = 5 + 10;
    println!("NaN: {}", sum);

    // 减法
    let difference = 95.5 - 4.3;
    println!("NaN: {}", difference);

    // 乘法
    let product = 4 * 30;
    println!("NaN: {}", product);

    // 除法
    let quotient = 56.7 / 32.2;
    println!("NaN: {}", quotient);

    // 求余
    let remainder = 43 % 5;
    println!("NaN: {}", remainder);
}

fn _t_char() {
    /*
       字符类型

       跟其他编程语言的字符可能不太一样，所有的 Unicode 值都可以作为 Rust 字符，包括单个的中文、日文、韩文、emoji 表情符号等等，都是合法的字符类型

       Unicode 值的范围从 U+0000 ~ U+D7FF 和 U+E000 ~ U+10FFFF
    */

    let c = 'z';
    println!("c: {}", c);
    let c = 'ℤ';
    println!("c: {}", c);

    let c = '国';
    println!("c: {}", c);

    let c = '😻';
    println!("c: {}", c);
}

fn _t_bool() {
    /*
       布尔

       Rust 中的布尔类型有两个可能的值：true 和 false，布尔值占用内存的大小为 1 个字节
    */

    let b = true;
    println!("true: {}", b);

    let b: bool = false; // 使用类型标注,显式指定f的类型
    println!("false: {}", b);

    if b {
        println!("这是段毫无意义的代码");
    }
}

fn _t_unit_type() {
    /*
       单元类型
    */
}
