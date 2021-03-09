fn main() {
    let mut args: Vec<String> = Vec::new();
    handle_arguments(&mut args);
    println!("{:?}", args);
}

fn handle_arguments(args: &mut Vec<String>) {
    for arg in std::env::args().skip(1) {
        println!("{}", arg);
        args.push(arg);
    }
}