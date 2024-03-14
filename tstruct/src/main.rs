#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

fn main() {
    // 声明一个struct变量
    let mut user1 = User {
        name: String::from("仔仔"),
        age: 25,
    };
    // 通过 .访问属性
    println!("user:{:#?}", user1);

    // 修改结构体的值
    user1.name = String::from("仔仔2");
    user1.age = 30;
    println!("user:{:#?}", user1)
}
