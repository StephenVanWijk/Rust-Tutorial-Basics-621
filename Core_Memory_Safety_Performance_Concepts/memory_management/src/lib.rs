// Memory management library root
pub fn print_usize_size() {
    println!("usize is {} bytes", std::mem::size_of::<usize>());
}