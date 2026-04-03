fn main() {
    my_library::main().unwrap_or_else(|e| {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    });
}