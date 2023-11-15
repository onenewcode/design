use std::fmt;

// 定义雇员
struct  Employee{
    name:String,
    dept:String,
    sal:i32,
    subordinates:Vec<Employee>
}
// 自定义格式化
impl fmt::Display for Employee {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f,"Employee : Name{}, dept : {} ,salary : {}", self.name,self.dept,self.sal)
    }
}
impl Employee {
    fn add(&mut self,e:Employee) {
        self.subordinates.push(e);
    }
    fn remove(&mut self,e:Employee) {
        self.subordinates.retain(|x| {
            if x.name!=e.name||x.dept==e.dept||x.sal==e.sal{
                return true;
            }
            return false;
        });
    }
    fn get_subordinates(&self) {
        self.subordinates.as_ptr();
    }
    fn new(name:String,dept:String, sal:i32)->Employee{
        Employee { name, dept,sal,subordinates:Vec::new() }
    }
    
}
fn pe(e:&Employee) {
    println!("{}",e);
    if !e.subordinates.is_empty(){
       e.subordinates.iter().for_each(|x| pe(x));
    }
    
}
fn main(){
    let mut ceo=Employee::new(String::from("John"), String::from("CEO"),30000);
    let mut head_sales=Employee::new(String::from("Robert"), String::from("Head Sales"),20000);
    let mut head_market=Employee::new(String::from("Michel"), String::from("Head Marketing"),10000);
    let mut clerk1=Employee::new(String::from("Laura"), String::from("Marketing"),10000);
    head_sales.add(head_market);
    head_sales.add(clerk1);
    ceo.add(head_sales);
    
    pe(&ceo)

}