trait  Iterator<T>{
    fn has_next()->bool;
    fn next()->T;
}
// #![feature(return_position_impl_trait_in_trait)]
trait  Container{
    fn getIterator()->impl Iterator;
}
fn main() {
    
}