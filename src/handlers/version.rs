
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");


pub fn version_handler(){
    println!("{}", VERSION);
}