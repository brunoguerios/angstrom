// We use jemalloc for performance reasons
#[cfg(all(feature = "jemalloc", unix))]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

fn main() {
    panic!("DOMAIN: {:?}", angstrom_types::primitive::ANGSTROM_DOMAIN);
    if let Err(err) = angstrom::run() {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }
}
