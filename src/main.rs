fn main() {
    // 测试数据类型
    //test_var();
    //test_scalar();
    test_compound_types();
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