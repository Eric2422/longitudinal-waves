use std::{env, fs};

mod particle;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1])?;
    fs::write(
        "output/".to_owned() + &args[1].split("/").collect::<Vec<&str>>()[1],
        &contents,
    )?;
    println!("{}", contents);

    Ok(())
}
