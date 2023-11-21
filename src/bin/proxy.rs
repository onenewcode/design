trait Image {
    fn dispaly(&self);
}
struct RealImage{
    file_name:String,
}
impl RealImage {
    fn load_from_disk(&self) {
        println!("Loading {}",self.file_name)
    }
    fn new(file_name:String)->RealImage {
        
        let i=RealImage { file_name:file_name.clone() };
        i.load_from_disk();
        i

    }
}

impl Image for RealImage {
    fn dispaly(&self) {
        println!("Displaying  {}",self.file_name.as_str())
    }
}
struct  ProxyImage{
    real_image: RealImage,
    file_name:String
    
}
impl ProxyImage {
    fn new(file_name:String)->ProxyImage {
        ProxyImage{
            real_image:RealImage::new(file_name.clone()),
            file_name:file_name
        }
            
    }
}
impl Image for ProxyImage {
    fn dispaly(&self) {
        self.real_image.dispaly();
    }
}
fn main() {
    let pi=ProxyImage::new("test_10mb.jpg".to_string());
    pi.dispaly();
    pi.dispaly();
}