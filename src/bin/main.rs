use std::{env, f32::consts::E};
use rayon::ThreadPoolBuilder;
use std::process::Command;

fn main() {

    let args: Vec<String> = env::args().collect();
    
    let programm = env::args().nth(1).unwrap_or("None".to_string());
    
    let threads = env::args().nth(2).unwrap_or("1".to_string());

    Command::new(programm)
        .arg(threads.to_string())
        .spawn()
        .expect("Ошибка запуска");
}
