fn main() {
    if let Err(e)=braincat::get_args().and_then(braincat::run){
        eprintln!("{}",e);
        std::process::exit(1);
    }
}
