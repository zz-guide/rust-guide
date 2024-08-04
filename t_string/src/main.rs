fn main() {
    // _t2();
    // _t3();
    // _t4();
    // _t5();
    // _t6();
    _t7();
}

fn _t7() {
    /*
    字符串转移和原生
     */

    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果还是有歧义，可以继续增加，没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
}

fn _t6() {
    /*
    字符串遍历 */

    // 字符串遍历，item为字符
    for c in "hello world".chars() {
        println!("{c}");
    }

    // 字符串遍历，item为字节
    for b in "中国人".bytes() {
        println!("{}", b);
    }

    // 获取子串，https://crates.io/crates/utf8_slice
}

fn _t5() {
    /*
    不支持索引读取
     */

    // let s1 = String::from("hello");
    // let h = s1[0];
    // println!("字符串的一部分: {}", h);
}

fn _t4() {
    /*
      push
       push_str
       insert
       insert_str
       replace
       replacen
       replace_range
       pop
       remove
       truncate
       clear
    */

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
}

fn _t2() {
    /*
       字符串的创建
    */

    let s = "hello,world!"; // &str类型
    println!("s: {}", s);

    let s = String::from(""); // String 类型
    println!("s: {}", s);

    let new_s = s.as_str(); // 转为 &str 类型
    println!("new_s: {}", new_s);

    let s = String::new(); // 通常用来创建一个空字符串
    println!("s: {}", s);
}

fn _t3() {
    /*
       字符串切片
       左开右闭,中间2个点
    */

    let s = String::from("hello");

    let slice = &s[0..2];
    println!("slice: {}", slice);

    let slice = &s[..2];
    println!("slice: {}", slice);

    let len = s.len();

    let slice = &s[0..len];
    println!("slice: {}", slice);

    let slice = &s[..];
    println!("slice: {}", slice);
}
