// 创建形状接口
trait Shape {
    fn draw(&self);
}
struct  Rectangle {}
struct Circle{}
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
// 创建装抽象接口
trait ShapeDecorator {
    // 装饰方式
    fn draw(&self);
}
// 创建装饰实现类
struct RedShapeDecorator{
    decorated_shape:Box<dyn Shape>
}
impl RedShapeDecorator {
    //设置修饰方法
    fn set_red_border(&self) {
        println!("Border Color: Red");
    }
}
// 实现装饰特征
impl  ShapeDecorator for RedShapeDecorator{
    fn draw(&self) {
        self.decorated_shape.draw();
        self.set_red_border();
    }
}
fn main() {
    let circle=Circle{};
    let red_circle=RedShapeDecorator{decorated_shape:Box::new(Circle{})};
    let red_rectangle=RedShapeDecorator{decorated_shape:Box::new(Rectangle{})};
    circle.draw();
    red_circle.draw();
    red_rectangle.draw();
}