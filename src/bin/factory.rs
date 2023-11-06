
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
    fn get_shape(shape_type: &str)->Option<Box<dyn Shape>>{
        if shape_type=="CIRCLE" {
            Some(Box::new(Circle{}))
        }else if shape_type=="RECTANGLE" {
            Some(Box::new(Rectangle{}))
            
        }else if  shape_type=="SQUARE"{
            Some(Box::new(Square{}))
            
        }else {
           None
        }
  
    }

}

fn main() {

    let shape1=ShapeFactory::get_shape("CIRCLE").unwrap();
    shape1.draw();
    let shape2=ShapeFactory::get_shape("RECTANGLE").unwrap();
    shape2.draw();
    let shape3=ShapeFactory::get_shape("SQUARE").unwrap();
    shape3.draw();
}
