fn main() {
    // 函数 return
    let name = get_name();
    println!("{}", name);

    // 函数 无 return，返回的表达式不能有分号
    let name = get_name1();
    println!("{}", name);
}

// 有return
fn get_name() -> String {
    return String::from("仔仔");
}

// 无 return
fn get_name1() -> String {
    String::from("仔仔")
}
