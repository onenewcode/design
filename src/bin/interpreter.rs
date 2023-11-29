// 声明表达式特征
trait  Expression{
    fn interpret(&self,context:&str)->bool ;
}
struct TerminalExpression{
    data:String,
}
impl Expression for TerminalExpression {
    fn interpret(&self,context:&str)->bool  {
        if context==self.data {
           return true; 
        }
        false
    }
}
// 创建或规则
struct OrExpression{
    expr1:Box<dyn Expression>,
    expr2:Box<dyn Expression>,
}
impl Expression for OrExpression {
    fn interpret(&self,context:&str)->bool  {
        self.expr1.interpret(context)||self.expr2.interpret(context)
    }
}
// 创建和规则
struct AndExpression{
    expr1:Box<dyn Expression>,
    expr2:Box<dyn Expression>,
}
impl Expression for AndExpression {
    fn interpret(&self,context:&str)->bool  {
        self.expr1.interpret(context)&&self.expr2.interpret(context)
    }
}
//规则：Robert 和 John 是男性
fn get_male_expression()->OrExpression{
    OrExpression { 
    expr1: Box::new(TerminalExpression{
        data:"Robert".to_owned()
    }), 
    expr2: Box::new(TerminalExpression{
        data:"John".to_owned()
    }),
 }
}
  //规则：Julie 是一个已婚的女性
fn get_married_woman_expression()->AndExpression{
    AndExpression { 
    expr1: Box::new(TerminalExpression{
        data:"Julie".to_owned()
    }), 
    expr2: Box::new(TerminalExpression{
        data:"Married".to_owned()
    }),
 }
}
fn main() {
    let is_male=get_male_expression();
    let is_married_woman=get_married_woman_expression();
    println!("John is male? {}",is_male.interpret("John"));
    println!("Julie is a married women? {}",is_married_woman.interpret("Married Julie"))
}