fn main() {
    if let Err(e)=brainr::get_args().and_then(brainr::run){
        eprintln!("{}",e);
        std::process::exit(1);
    }
}
