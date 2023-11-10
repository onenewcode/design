// 画图的接口
trait DrawAPI {
    fn draw_circle(&self,radius:i32,x:i32, y:i32);      
}
// 画h
struct RedCircle {}
impl DrawAPI for RedCircle {
    fn draw_circle(&self,radius:i32,x:i32, y:i32) {
        println!("Drawing Circle[ color: red, radius: {}, x: {}, {}]",radius,x,y);
    }
}
struct GreenCircle {}
impl DrawAPI for GreenCircle  {
    fn draw_circle(&self,radius:i32,x:i32, y:i32) {
        println!("Drawing Circle[ color: green, radius: {}, x: {}, {}]",radius,x,y);
    }
}
trait Shape{
    fn draw(&self);
}
struct Circle{
    // // pub(crate) 使得函数只在当前 crate 中可见
    // 抽象方法的实现
    draw_api:Box<dyn DrawAPI>,
    x:i32,
    y:i32,
    radius:i32
}
impl Circle {
    fn new(draw_api:Box<dyn DrawAPI>,x:i32, y:i32, radius:i32)->Circle {
        Circle{
            draw_api,
            x,
            y,
            radius
        }
    }
}
impl Shape for Circle  {
    fn draw(&self) {
        self.draw_api.draw_circle(self.radius, self.x, self.y)
    }
}
fn main(){
    let r=Circle::new(Box::new(RedCircle{}),100,100, 10);
    let g=Circle::new(Box::new(GreenCircle{}),100,100, 10);
    r.draw();
    g.draw();

}