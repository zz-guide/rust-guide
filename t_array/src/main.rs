fn main() {
    _t1();
}

fn _t1() {
    /*
       数组
       长度固定，类型相同的元素的集合。
    */

    let a = [1, 2, 3, 4, 5];
    println!("a: {:?}", a);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a: {:?}", a);

    let a = [3; 5];
    println!("a: {:?}", a);

    let array: [String; 8] = std::array::from_fn(|_i| String::from("rust is good!"));
    println!("{:#?}", array);
}

fn _t2() {
    /*
    数组切片 */

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let slice: &[i32] = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
