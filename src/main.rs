fn main() {
    // 测试数据类型
    //test_var();
    //test_scalar();
    //test_compound_types();
    //let  result = test_fn(23333,true);
    //println!("fn comeback {}",result);
    //test_control_flow();
    //test_fn_index();
    test_slice();
}


//slice 切片
fn test_slice( ) {
    let str = String::from("cai jiasheng");
    let cai = &str[0..3];
    let name = &str[3..];
    //println!("{}  {}",cai,name);
    let result = slice_fist_world(&str);
    println!("{}",result);
    //str.clear();
}

fn slice_fist_world (s:&str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// 引用规则
// 在作用域之内，只能拥有下面的一个场景 1、一个可变引用&mut 2、任意数量的不可变引用&
// 引用必须总是有效的，不能返回指向内存已经释放的指针

fn test_fn_index() {
    let mut  test = String::from("test");
    //let (str, len) = test_fn2(test);
    //println!("{} length {}",str,len);
    //println!("{}",test_fn_ref(&test));
    //println!("{}",test_fn_change_ref(&mut test));
    //test_multiple_ref();
    test_ref_change_nochange();
}
//不能在拥有不可变变量同时，拥有可变变量
fn test_ref_change_nochange(){
    let mut s = String::from("caijiasheng");
    let r1 = &s;
    let r2 = &s;
    //let r3 = &mut s;
    //println!("{} {} {}",r1,r2,r3);
}

fn test_multiple_ref(){
    let mut s = String::from("caijiasheng");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;
    // let r3 = &mut s; 不允许多次使用引用可变变量
    // println!(" r2 {} \n r3 {}",r2,r3);

}

//测试变量的引用 使用&符号，而且是在变量类型上不是在变量上，这里传递的是一个指针，使用了同一个内存数据，但只是借用的，所以在作用域结束后不会被释放掉
// 对于通过指针借用（引用）的内存数据，不允许修改，只能读取
fn test_fn_ref(str:&String) -> usize{
    return str.len();
}

//可变引用
// 需要变量定义、函数定义、还有函数使用入口都要加上&mut 可变变量引用
// 一个作用域内只允许定义一个 &mut参数，
fn test_fn_change_ref (str:&mut String) -> usize {
    //同时字符串不允许值
    str.push_str(" caijiasheng");// 字符串连接
    return str.len();
}


// 测试函数返回元组，变量move场景
fn test_fn2(s: String) -> (String, usize) {
    let len = s.len();
    // return (s, s.len()); 不能直接这么使用，因为s已经move到元组中，不能继续调用其属性
    return (s, len);
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