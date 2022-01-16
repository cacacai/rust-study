fn main() {
    // 测试数据类型
    //test_var();
    //test_scalar();
    //test_compound_types();
    //let  result = test_fn(23333,true);
    //println!("fn comeback {}",result);
    test_control_flow();
}
// 测试使用控制流
fn test_control_flow() {
    let num = 3;
    if num > 5 {//必须为bool值，或者判断条件
        println!("{}", num);
    } else {
        println!("{}", num);
    }

    //
    let condition = true;
    let number = if condition { 5 } else { 6 };//控制语句中没有分号，整体为一个表达式
    println!("{}", number);


    loop {
        println!("loop 无限循环");
        break;
    }

    let mut i = 0;
    while i < 4 {
        println!("{}", i);
        i = i + 1;
    }

    let a = [1, 2, 3, 4, 5];
    for element in a {
        println!("{}",element);
    }
    for i in (20..29).rev() {//反转数组
        println!("rev {}",i);
    }
}

fn test_fn(x:i32,y:bool)-> i32{
    if y {
        println!("{}",x);
    }
    return 6666;
}


fn test_var() {
    let x = 7;
    let x = x + 1;
    let x = x * 2;
    println!("value {}", x);
}
// 标量类型  数字 浮点数 bool 字符
fn test_scalar() {
    let dec = 89_222;//__只是为了方便读数，无任何意义
    print!("{}",dec);
    let x = 2.4;//双精度 默认
    let y:f32 = 2.0;//单精进

    let t = true;
    let f:bool = false;

    let c = 'c';
    let c = '🤣';//支持unicode字符
}
// 组合类型 元组 数组
fn test_compound_types(){
    let tup:(i32,f64,u8) = (399,2.1,1);//元组
    let (x,y,z) = tup;//解构
    println!("the value of y is {}",tup.1);//索引获取元组中的成员

    let a = [1,2,3];
    print!("{}",a[1]);

}