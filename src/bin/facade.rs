// 创建形状接口
trait Shape {
    fn draw(&self);
}
struct  Rectangle {}
struct Circle{}
struct Square{}
impl Shape for Rectangle {
    fn draw(&self) {
        println!("Shape: Rectangle");
    }
}
impl Shape for Circle {
    fn draw(&self) {
        println!("Shape: Circle");
    }
}
impl Shape for Square {
    fn draw(&self) {
        println!("Shape: Square");
    }
}
// 创建外观
struct ShapeMaker{
    rectangle:Rectangle,
    circle:Circle,
    square:Square
}
impl ShapeMaker {
    fn draw_rectangle(&self) {
        self.rectangle.draw();
    }
    fn draw_circle(&self) {
        self.circle.draw();
    }
    fn draw_square(&self) {
        self.square.draw();
    }
}
fn main() {
    //创建接口实体
    let shape_maker=ShapeMaker{rectangle:Rectangle {  },circle:Circle {  },square:Square {  }};
    // 体现接口抽象实现的各种方法
    shape_maker.draw_circle();
    shape_maker.draw_rectangle();
    shape_maker.draw_square();
}