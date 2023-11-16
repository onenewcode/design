use std::{collections::HashMap};
use rand::{Rng};
// 创建形状接口
trait Shape {
    fn draw(&self);
}
struct Circle{
    color:String,
    x:i32,
    y:i32,
    radius:i32,
}
impl Shape for Circle {
    fn draw(&self) {
        println!("Circle: Draw() [Color: {},x: {},y: {},radius: {}]",self.color,self.x,self.y,self.radius);
    }
}
impl Circle {
    fn new(color:String)->Circle {
        Circle{
            color:color,
            x:0,
            y:0,
            radius:0
        }
    }
}
// 设置形状工场，进行管理
struct ShapeFactory{
    circle_map:HashMap<String,Circle>
} 
impl ShapeFactory {
    fn get_circle(&mut self,color:&str)->&mut Circle{

        match self.circle_map.get(color) {
            None=>{
                self.circle_map.insert(color.to_owned(),Circle::new(color.to_owned()));  
                println!("Creating circle of color : {}" ,color);
            }
            Some(_)=>{
                println!("已有触发享元")
            }
        };
        self.circle_map.get_mut(color).unwrap()
    }
}
fn get_rand_color()->& 'static str {
    COLORS[rand::thread_rng().gen_range(0..COLORS.len())]
}
fn get_randx()->i32 {
    rand::thread_rng().gen_range(0..100)
}
fn get_randy()->i32 {
    rand::thread_rng().gen_range(0..100)
}
const  COLORS: [&str; 5]=["Red", "Green", "Blue", "White", "Black" ];

fn main() {
  
    let  mut shape_factory=ShapeFactory{circle_map:HashMap::new()};
    for _x in 1..=5 {
        let  color=shape_factory.get_circle(get_rand_color());
        color.x=get_randx();
        color.y=get_randy();
        color.radius=100;
        color.draw();
    }
    // println!("{}",get_rand_color());
        
    

}