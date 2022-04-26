use builder::son;

fn main(){
    //read second argument
    let args: Vec<String> = std::env::args().collect();
    son::process_son_def(&args[1], &args[2]);
}