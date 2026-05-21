use std::path::Path;
use annihilation::{logo, directory, confirm, step_1, step_2};

fn main() {
    
    //Start
    logo();
    let dir = directory();
    confirm();

    //Working
    let is_success = step_1(Path::new(&dir));
    step_2(&dir, is_success);

    if is_success {
        println!("{} IS DEAD FOR EVER AND EVER.", dir.to_uppercase());
    } else {
        eprintln!("WARNING: Some files were locked. Partial wipe completed.");
    }
}