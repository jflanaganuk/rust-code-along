use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let arg1 = args.get(1);
    if arg1.is_none(){
        println!("No arg provided");
    } else if arg1.unwrap() == "hello" {
        println!("You said hello!");
    } else {
        println!("You didn't say hello :( !");
    }

    println!("{:?}", arg1);
}