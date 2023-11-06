
// 定义接口
pub  trait Shape {
    fn draw(&self);  
}
// 创建实体类
struct Rectangle;
struct Square;
struct  Circle;
impl Shape for Rectangle {
    fn draw(&self) {
        println!("Inside Rectangle::draw() method.");
    }
}
impl Shape for Square {
    fn draw(&self) {
        println!("Inside Square::draw() method.");
    }
}
impl Shape for Circle {
    fn draw(&self) {
        println!("Inside Circle::draw() method.");
    }
}
struct ShapeFactory ;
impl ShapeFactory {
    fn get_shape(shape_type: &str)->Box<dyn Shape>{
        if shape_type=="CIRCLE" {
            Box::new(Circle{})
        }else if shape_type=="RECTANGLE" {
            Box::new(Rectangle{})
        }else if  shape_type=="SQUARE"{
            Box::new(Square{})
        }else {
            panic!("输入的类型不存在");
        }
  
    }

}

fn main() {

    let shape1=ShapeFactory::get_shape("CIRCLE");
    shape1.draw();
    let shape2=ShapeFactory::get_shape("RECTANGLE");
    shape2.draw();
    let shape3=ShapeFactory::get_shape("SQUARE");
    shape3.draw();
}
