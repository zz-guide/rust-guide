fn main() {
    // _t();
    _t1();
}

fn _t() {
    /*
    元组（Tuple）是一种固定长度的有序集合，其元素可以是不同类型。元组可以作为函数的返回值，也可以作为函数的参数。
    */

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup: {:?}", tup);

    // 解构元组
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {} {} {}", x, y, z);
}

fn _t1() {
    /*
       访问元组
    */

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    println!("The value of five_hundred is: {}", five_hundred);

    let six_point_four = x.1;
    println!("The value of six_point_four is: {}", six_point_four);

    let one = x.2;
    println!("The value of one is: {}", one);
}
