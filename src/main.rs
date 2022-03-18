fn main() {
    if let Err(e) = rust_head::get_args().and_then(rust_head::run) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
