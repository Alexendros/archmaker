fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match archmaker_constitute::run_args(&args) {
        Ok(body) => {
            println!("{body}");
        }
        Err(error) => {
            println!("{}", error.to_json());
            std::process::exit(1);
        }
    }
}
