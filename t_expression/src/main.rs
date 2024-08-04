fn main() {
    _main();
}

fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式
}

fn _t() {
    /*
       表达式和与的区别

       Rust中函数的最后一行如果是一个表达式的话，会自动求值作为返回值，并且表达式后边是没有分号的
    */

    let res = add_with_extra(1, 2);
    println!("res = {}", res);
}

fn _main() {
    // _t();

    dead_end();
}

fn dead_end() -> ! {
    panic!("你已经到了穷途末路，崩溃吧！");
}
