use metal_cpp_rs::metal_cpp_rs_bindings::root::*;

fn main() {
    unsafe {
        let pool = NS::AutoreleasePool::alloc();
        println!("hello world!");
        (&mut *pool).init();
    }
}
