fn main() {
    // 1. 定义一个字符串
    let name1 = String::from("仔仔1");
    println!("姓名1: {}", name1);

    // 2. 创建一个字符串
    let name2 = String::new();
    println!("姓名2: {}", name2);

    // 3.
    let name3 = "仔仔3";
    println!("姓名3: {}", name3.to_string());

    // 字符串追加内容 push_str
    let mut name4 = String::from("仔仔4");
    name4.push_str("仔仔4");
    println!("姓名4: {}", name4);

    // 字符串追加内容 push
    let mut name5 = String::from("仔仔5");
    name5.push('仔');
    name5.push('仔');
    name5.push('5');
    println!("姓名4: {}", name5);

    // + 组合字符串
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("组合字符串: {}", s);

    // format 宏 组合字符串
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("组合字符串: {}", s);

    // 不支持索引读取
    // let s1 = String::from("hello");
    // let h = s1[0];
    // println!("字符串的一部分: {}", h);

    // 引用切片, 左开右闭,中间2个点
    let hello = "hello world";
    let s = &hello[0..4];
    println!("字符串的一部分: {}", s);

    // 字符串遍历，item为字符
    for c in "hello world".chars() {
        println!("{c}");
    }
}
