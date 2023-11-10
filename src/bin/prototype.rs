#[derive(Clone)]
struct Shape{
    id:String,
    mtype:String
}
impl Shape{
    fn set_id(&mut self,id:String){
        self.id=id;
    }
    fn get_id(&self)->&str{
        &self.id
    }
}
#[derive(Clone)]
struct  Rectangle{
    shape:Shape
}
impl Rectangle {
    fn new()->Rectangle {
        Rectangle{shape:Shape{
            id:String::from("value"),
            mtype:String::from("Rectangle")
        }}
    }
    fn draw() {
        println!("Inside Rectangle::draw() method.");
    }
}
#[derive(Clone)]
struct  Square {
    shape:Shape
}
impl Square  {
    fn new()->Square  {
        Square {shape:Shape{
            id:String::from("value"),
            mtype:String::from("Square ")
        }}
    }
    fn draw() {
        println!("Inside Square ::draw() method.");
    }
}
#[derive(Clone)]
struct  Circle {
    shape:Shape
}
impl Circle  {
    fn new()->Circle {
        Circle {shape:Shape{
            id:String::from("value"),
            mtype:String::from("Square ")
        }}
    }
    fn draw() {
        println!("Inside Circle ::draw() method.");
    }
}

fn main(){
    let s=Circle::new();
    let mut s1=s.clone();
    s1.shape.set_id(String::from("dsf"));
    println!("{}",s.shape.id);
    println!("{}",s1.shape.id);

}