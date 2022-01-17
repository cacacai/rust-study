fn main() {
    //let width = 30;
    //     let height = 50;
    //把数据封装为元组
    let rect1 = (30, 50);
    println!("area {}",area_mate(rect1));


}

fn area_mate(dimensions: (u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(height: u32, width: u32) -> u32 {
    height * width
}