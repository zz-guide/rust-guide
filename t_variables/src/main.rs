fn main() {
    // _t1()
    // _t2()
    // _t3()
    // _t4()
    _t5()
}

fn _t1() {
    /*
      绑定变量，默认是不可变的
    */
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // 报错，cannot assign twice to immutable variable
    println!("The value of x is: {}", x);
}

fn _t2() {
    /*
      加上 mut 关键字，可以修改变量的值
    */
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn _t3() {
    /*
      变量解构
    */
    let (a, mut b): (bool, bool) = (true, false);

    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);
}

fn _t4() {
    /*
      变量解构结构体
    */

    struct Struct {
        e: i32,
    }

    let (a, b, c, d, e);

    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}

fn _t5() {
    /*
     变量遮蔽
    */

    let x = 5;
    // 在main函数的作用域内对之前的x进行遮蔽
    let x = x + 1; // x = 6
    println!("x + 1: {}", x);
    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2; // 还能访问到外边的x
        println!("The value of x in the inner scope is: {}", x); // 12
    }

    println!("The value of x is: {}", x); // 访问不到上边方法块中的x
}
