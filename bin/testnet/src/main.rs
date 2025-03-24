// We use jemalloc for performance reasons
#[cfg(all(feature = "jemalloc", unix))]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

fn main() {
    println!("starting angstrom...");
    println!("installing ctrl+c handler...");
    ctrlc::set_handler(move || {
        println!();
        println!("GOT IT");
        println!();
    })
    .expect("error setting ctrl+c handler");
    println!("ctrl+c handler installed.");
    if let Err(err) = testnet::run() {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }
}
