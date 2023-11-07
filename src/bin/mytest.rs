use std::sync::{Arc, Mutex};
 struct Singleton {
    // 单例数据
    data: String,
}
 impl Singleton {
    // 获取单例实例的方法
    fn get_instance() -> Arc<Mutex<Singleton>> {
        // 使用懒加载创建单例实例
        // 这里使用了 Arc 和 Mutex 来实现线程安全的单例
        // 只有第一次调用 get_instance 时会创建实例，之后都会返回已创建的实例
        static mut INSTANCE: Option<Arc<Mutex<Singleton>>> = None;
         unsafe {
            INSTANCE.get_or_insert_with(|| {
                Arc::new(Mutex::new(Singleton {
                    data: String::from("Singleton instance"),
                }))
            }).clone()
        }
    }
}
 fn main() {
    // 获取单例实例
    let instance1 = Singleton::get_instance();
    let instance2 = Singleton::get_instance();
     // 修改单例数据
    {
        let mut instance = instance1.lock().unwrap();
        instance.data = String::from("Modified singleton instance");
    }
     // 输出单例数据
    {
        let instance = instance2.lock().unwrap();
        println!("{}", instance.data);
    }
}
