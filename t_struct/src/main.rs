fn main() {
    // _t();
    // _t2();
    _t3();
}

fn _t() {
    /*
       创建结构体变量
    */

    #[derive(Debug)]
    struct User {
        name: String,
        age: u8,
    }

    let user1 = User {
        name: String::from("仔仔"),
        age: 25,
    };

    // 通过 .访问属性
    println!("user:{:#?}", user1);
    println!("user.name:{:#?}", user1.name);
    println!("user.age:{:#?}", user1.age);
}

fn _t2() {
    /*
    结构体必须整体可变，不支持某个属性可变
    使用 #[derive(Debug)] 来打印结构体的信息
     */

    #[derive(Debug)]
    struct User {
        name: String,
        age: u8,
    }

    let mut user1 = User {
        name: String::from("仔仔"),
        age: 25,
    };

    // 修改结构体的值
    user1.name = String::from("仔仔2");
    user1.age = 30;
    println!("user:{:#?}", user1) // {:?}
}

fn _t3() {
    /*
       元组结构体
    */

    #[derive(Debug)]
    struct Color(i32, i32, i32);

    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    println!("black:{:#?}", black);

    let origin = Point(0, 0, 0);
    println!("origin:{:#?}", origin);
}

fn _t4() {
    /*
    单元结构体
     */

    // struct AlwaysEqual;

    // let subject = AlwaysEqual;

    // 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
    // impl SomeTrait for AlwaysEqual {}
}
