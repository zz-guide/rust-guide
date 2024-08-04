fn main() {
    // _t1();
    _t2();
}

fn _t6() {
    /*
    loop
     */
}

fn _t5() {
    /*
    while
     */
}

fn _t4() {
    /*
    continue
     */
}

fn _t3() {
    /*
    break
     */
}

fn _t2() {
    /*
       for
    */

    for i in 1..=5 {
        println!("{}", i);
    }

    /*
     *
     * for item in collection  for item in IntoIterator::into_iter(collection)  转移所有权
     * for item in &collection	for item in collection.iter()	不可变借用
     * for item in &mut collection	for item in collection.iter_mut()	可变借用
     */

    // 迭代过程中获取索引
    let a = [4, 3, 2, 1];
    // `.iter()` 方法把 `a` 数组变成一个迭代器
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }
}

fn _t1() {
    /*
       if else
    */
    let n = 6;

    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
