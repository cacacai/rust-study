//添加注解为了直接调试，打印对象
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
// 实现自身的解构体，添加自身的方法函数
impl Rectangle {
    fn area(&self) -> u32 {//self 传入自身对象，类型于this访问；使用&表示借用，不可变
        self.width * self.width
    }
    //比较整体大小
    fn can_hold(&self,other:&Rectangle) -> bool {
        self.width >other.width && self.height > other.height
    }
}

fn main() {
    //let width = 30;
    //     let height = 50;
    //把数据封装为元组
    //let rect1 = (30, 50);
    //println!("area {}",area_mate(rect1));

    let rect1 = Rectangle { width: 30, height: 20 };
    //println!("area {}",area_struct(&rect1));
    //println!("area {:?}",rect1);
    //println!("area {}",rect1.area());
    let rect1 = Rectangle { width: 30, height: 20 };
    let rect2 = Rectangle { width: 40, height: 20 };
    let rect3 = Rectangle { width: 50, height: 60 };
    println!("rect1 > rect2? {}", rect1.can_hold(&rect2));
    println!("rect3 > rect2? {}", rect3.can_hold(&rect2));



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