

fn main() {
    //let width = 30;
    //     let height = 50;
    //把数据封装为元组
    //let rect1 = (30, 50);
    //println!("area {}",area_mate(rect1));

    let rect1 = Rectangle{
        width:30,height:20
    };
    //println!("area {}",area_struct(&rect1));
    println!("area {:?}",rect1);



}
//添加注解为了直接调试，打印对象
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
//通过结构体计算，同时使用 & 借用，引用方式
fn area_struct(rect:&Rectangle) -> u32 {
    rect.height * rect.width
}
fn area_mate(dimensions: (u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(height: u32, width: u32) -> u32 {
    height * width
}