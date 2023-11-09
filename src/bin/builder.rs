// rsut trait不支持重名
pub trait Item {
    fn name(&self)->String;   
    fn price(&self)->f32;   
    fn packing(&self)->String;
}

// 汉堡实体类
struct ChickenBurger;
impl Item for ChickenBurger {
    fn name(&self)->String {
        String::from("ChickenBurger")
    }
    fn price(&self)->f32 {
        35.0
    }
    fn packing(&self)->String {
        String::from("Wrappper")
    }
   
}  
struct VegBurger;
impl Item for VegBurger {
    fn name(&self)->String {
        String::from("Pepsi")
    }
    fn price(&self)->f32 {
        35.0
    }
    fn packing(&self)->String {
        String::from("Wrappper")
    }
}  

// 饮料实体类
struct Pepsi;

impl Item for Pepsi {
    fn name(&self)->String {
        String::from("Pesi")
    }
    fn price(&self)->f32 {
        35.0
    }
    fn packing(&self)->String {
        String::from("Bottle")
    }
}  
struct Coke;

impl Item for Coke {
    fn name(&self)->String {
        String::from("Coke")
    }
    fn price(&self)->f32 {
        35.0
    }
    fn packing(&self)->String {
        String::from("Bottle")
    }
}  

struct Meal {
    items:Vec<Box<dyn Item>>,
}
impl Meal {
    fn add_item<T>(&mut self,item:Box<dyn Item>) 
    {
        self.items.push(item)
    }
    fn get_cost(&self)->f32{
        // 普通函数实现
        // let mut sum:f32=0.0; 
        // for i in self.items.iter()  {
        //     let price = i.price();
        //     sum+=price;
        // }
        // sum

        // 函数sji
        self.items.iter().fold(0.0, |acc, x| acc + x.price())
    }
    fn  show_items(&self){
        for  i in self.items.iter() {
            println!("{}",i.name());
            println!("{}",i.packing());
            println!("{}",i.price());
        }
    }
}
// 添加建造者
struct MealBuilder {}
impl  MealBuilder {
    fn prepareVegMeal()->Meal{
        let mut meal=Meal{items:Vec::new()};
        meal.add_item::<Box<dyn Item>>(Box::new(VegBurger{}));
        meal.add_item::<Box<dyn Item>>(Box::new(Coke{}));
        meal
    }
    fn prepareNonVegMeal()->Meal{
        let mut meal=Meal{items:Vec::new()};
        meal.add_item::<Box<dyn Item>>(Box::new(ChickenBurger{}));
        meal.add_item::<Box<dyn Item>>(Box::new(Pepsi{}));
        meal
    }
}
fn main() {
    let m=MealBuilder::prepareVegMeal();
    println!(" Veg Meal");
    m.show_items();
    println!("Total Cost : {}",m.get_cost());
    let m=MealBuilder::prepareNonVegMeal();
    println!(" \n\nNon-Veg Meal");
    m.show_items();
    println!("Total Cost : {}",m.get_cost());
}