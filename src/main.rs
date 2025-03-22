fn main() {
    if let Err(e)=braincats::get_args().and_then(braincats::run){
        eprintln!("{}",e);
        std::process::exit(1);
    }
}
