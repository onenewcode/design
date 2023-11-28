
const INFO:i32=1;
const DEBUG:i32=2;
const ERROR:i32=3;
trait AbstractLogger {
    fn write(&self,message: &str);
}

struct Logger{
    level:i32,
    next_logger: Box<dyn AbstractLogger>
}

impl  Logger {
    fn set_next_logger(&mut self,next_logger:Box<dyn AbstractLogger>){
        self.next_logger=next_logger;
    }
    fn log_message(&self,level:i32,message:&str){
        if self.level<=level{
            self.next_logger.write(message);
        }
       if

    }
}

fn main() {
    
}