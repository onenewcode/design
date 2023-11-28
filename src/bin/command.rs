trait Order {
    fn execute(&self);
}
#[derive(Clone)]
struct Stock{
    name:String,
    quantity:i32,
}
impl Stock {
    fn buy(&self) {
        println!("Stock [ Name : {}  Quantity: {}] bought",self.name,self.quantity);
    }
    fn sell(&self) {
        println!("Stock [ Name : {}  Quantity: {}] sold",self.name,self.quantity);
    }
    fn new()->Stock{
        Stock{
            name:"ABC".to_owned(),
            quantity:10
        }
    }
}
// 创建命令实体
struct  BuyStock {
    abc_stock:Stock
}
impl Order for BuyStock {
    fn execute(&self) {
        self.abc_stock.buy();
    }
}
struct  SellStock {
    abc_stock:Stock
}
impl Order for SellStock {
    fn execute(&self) {
        self.abc_stock.sell();
    }
}
// 创建命令调用类。
struct Broker{
    order_list:Vec<Box<dyn Order>>
}
impl Broker {
    fn take_order(&mut self,order:impl Order+ 'static){
        
        self.order_list.push(Box::new(order));
    }
    fn place_orders(&self){
        self.order_list.iter().for_each(|f|f.execute());
    }
    fn  new()->Broker {
        Broker{
            order_list:vec![]
        }
    }
}
fn main(){
    let stock=Stock::new();
   
    let buy=BuyStock{abc_stock:stock.clone()};
    let sell=SellStock{abc_stock:stock.clone()};
    let mut broker=Broker::new();
    broker.take_order(buy);
    broker.take_order(sell);
    broker.place_orders();

}