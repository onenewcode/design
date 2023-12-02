// 设置备忘录类
#[derive(Clone)]
struct Memento{
        state:String
}
// 设置创始类
struct Originator{
    state:String,
}
impl Originator {
    fn save_state_to_memento(&self)->Memento{
        Memento { state: self.state.clone() }
    }
    fn get_state_from_memento(&mut self,memento:Memento){
        self.state=memento.state.clone();
    }
}
// 创建看护人类
struct CareTaker{
    memento_list:Vec<Memento>
}
impl CareTaker {
    fn add(&mut self,memento:Memento){
        self.memento_list.push(memento);
    }
    fn get(&self,index:usize)->Memento{
        self.memento_list.get(index).unwrap().clone()
    }
}

fn main() {
    let mut originator=Originator{state:"State #1".to_owned()};
    let mut careTaker =CareTaker{memento_list:vec![]}; 
    originator.state=String::from("State #2");
    careTaker.add(originator.save_state_to_memento());
    originator.state=String::from("State #3");
    careTaker.add(originator.save_state_to_memento());
    originator.state=String::from("State #4");
    println!("Current State: {}",originator.state);
    originator.get_state_from_memento(careTaker.get(0));
    println!("First saved State: {}",originator.state);
    originator.get_state_from_memento(careTaker.get(1));
    println!("Second saved State: {}",originator.state)
}