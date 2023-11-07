use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
 struct Singleton;
 impl Singleton {
    //关联方法， 获取单例实例的方法
    fn get_instance() -> Arc<Mutex<Singleton>> {
        // 使用懒加载创建单例实例
        // 这里使用了 Arc 和 Mutex 来实现线程安全的单例
        // 只有第一次调用 get_instance 时会创建实例，之后都会返回已创建的实例
        static mut INSTANCE: Option<Arc<Mutex<Singleton>>> = None;//静态初始化，只运行一次
         unsafe {
            // get_or_insert_with ,如果是 None ，则将从data计算的值插入选项中，然后返回对包含值的可变引用。
            INSTANCE.get_or_insert_with(|| {
                Arc::new(Mutex::new(Singleton {}))}).clone()
        }
    }
    fn show_message(&self){
        println!("Hello World!");
     }
}

// 使用lazy_static的懒加载
struct SingletonL;
impl SingletonL {
    fn show_message(&self){
        println!("Hello World!");
     }
}
lazy_static! {
    static ref INSTANCE: Mutex<SingletonL> = Mutex::new(SingletonL {   });
}

fn main() {
  // 获取单例实例,自定义
  let instance1 = Singleton::get_instance();
  
   // 修改单例数据
  {
      let instance = instance1.lock().unwrap();
      instance.show_message();
  }
  // 获取单例实例,社区lazy_static
  let instance = INSTANCE.lock().unwrap();
  instance.show_message();
}