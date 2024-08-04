fn main() {
    _t1();
}

fn _t1() {
    let mut v: Vec<i32> = Vec::new();
    println!("v: {:?}", v);

    v.push(1);
    println!("v: {:?}", v);

    // 使用宏 vec! 来创建数组
    let v = vec![1, 2, 3];
    println!("v: {:?}", v);
}
