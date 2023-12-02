use chrono::Local;
struct  ChatRoom{}
impl ChatRoom {
    fn show_message(user:&User,message:&str) {
        println!("{}, [ {} ] {}",Local::now(),user.name,message);
    }
}
struct User{
    name:String,
}
impl User{
    fn send_message(&self,message:&str) {
    ChatRoom::show_message(self, message)
}
}
fn main() {
    let rober=User{
        name:String::from("Robert"),
    };
    let john=User{
        name:String::from("John")
    };
    rober.send_message("Hi! John!");
    john.send_message("Hello! Robert!");
}