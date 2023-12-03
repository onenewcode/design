// 创建策略特征
trait  Strategy{
   fn  do_operation(&self,num1:i32,num2:i32)->i32;
}

struct OperationAdd{
}
// 创建加法策略
impl Strategy for OperationAdd {
    fn  do_operation(&self,num1:i32,num2:i32)->i32 {
        num1+num2
    }
}
// 创建减法测率
struct OperationSubtract{}
impl Strategy for OperationSubtract{
    fn  do_operation(&self,num1:i32,num2:i32)->i32 {
        num1-num2
    }
}
// 创建乘法策略
struct OperationMultiply{}
impl Strategy for OperationMultiply {
    fn  do_operation(&self,num1:i32,num2:i32)->i32 {
        num1*num2
    }
}
// 创建环境，管理策略方法
struct Context{
    strategy:Box<dyn Strategy>,
}
impl Context {
    fn execute_strategy(&self,num1:i32,num2:i32)->i32 {
        self.strategy.do_operation(num1, num2)
    }
}
fn main() {
    let mut context=Context{
        strategy:Box::new(OperationAdd{}),
    };
    println!("10 + 5 =  {}",context.execute_strategy(10, 5));
    context.strategy=Box::new(OperationSubtract{});
    println!("10 - 5 =  {}",context.execute_strategy(10, 5));
    context.strategy=Box::new(OperationMultiply{});
    println!("10 * 5 =  {}",context.execute_strategy(10, 5));
}