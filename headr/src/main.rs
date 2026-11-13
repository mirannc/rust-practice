fn main() {
    if let Err(e) = headr::get_args().ans_then(headr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
