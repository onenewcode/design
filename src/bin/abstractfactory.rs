// 定义接口

// 创建实体类
struct Rectangle;
struct Square;
struct  Circle;
struct ShapeFactory ;
struct ColorFactory ;
struct  FactoryProducer ;
struct Red;
struct Blue;
struct  Green;
pub trait Color{
    fn fill(&self);  
}
impl Color for Red {
    fn fill(&self) {
        println!("Inside Red::fill() method.");
    }
}
impl Color for Blue {
    fn fill(&self) {
        println!("Inside Blue::fill() method.");
    }
}
impl Color for Green {
    fn fill(&self) {
        println!("Inside Green::fill() method.");
    }
}


pub  trait Shape {
    fn draw(&self);  
}
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
pub trait AbstractFactory {
    fn get_color(&self,color: &str)->Box<dyn Color>;
    fn get_shape(&self,shape: &str)->Box<dyn Shape>;
 }

impl AbstractFactory for ShapeFactory {
    fn get_shape(&self,shape_type: &str)->Box<dyn Shape>{
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
    fn get_color(&self,_shape_type: &str)->Box<dyn Color>{

        panic!("输入的类型不存在");
  
    }
}
impl AbstractFactory for ColorFactory {
    fn get_color(&self,shape_type: &str)->Box<dyn Color>{
        if shape_type=="RED" {
            Box::new(Red{})
        }else if shape_type=="BLUE" {
            Box::new(Blue{})
        }else if  shape_type=="GREEN"{
            Box::new(Green{})
        }else {
            panic!("输入的类型不存在");
        }
    }
    fn get_shape(&self,_shape_type: &str)->Box<dyn Shape>{

        panic!("输入的类型不存在");
    }

}

impl  FactoryProducer {
    fn get_factory(choice: &str)-> Box<dyn AbstractFactory>{
        if choice=="COLOR" {
            Box::new(ColorFactory{})
        }else if choice=="SHAPE" {
            Box::new(ShapeFactory{})
        }else {
            panic!("输入的类型不存在");
        }
    }
}

fn main() {
    let shape=FactoryProducer::get_factory("SHAPE");
    let shape1=shape.get_shape("CIRCLE");
    shape1.draw();
    let shape2=shape.get_shape("RECTANGLE");
    shape2.draw();
    let shape3=shape.get_shape("SQUARE");
    shape3.draw();
    let color=FactoryProducer::get_factory("COLOR");
    let color1=color.get_color("RED");
    color1.fill();
    let color2=color.get_color("BLUE");
    color2.fill();
    let color3=color.get_color("GREEN");
    color3.fill();
}