use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process::Command;
use std::env;
use std::fs;

fn bash(file_parameter: String){
    let contents = fs::read_to_string(file_parameter.clone())
        .expect("Something went wrong reading the file");

    // println!("With text:\n{}", contents);

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", "echo hello-world"])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(contents.clone())
                .output()
                .expect("failed to execute process")
    };

    let result = String::from_utf8_lossy(&output.stdout);
    // println!("stdout: {}", result);
    assert!(output.status.success());
    send_and_write(file_parameter.to_string().clone(), result.to_string(), contents.clone());
}

fn send_and_write(file_parameter: String, resultat: String, to_execute: String){
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_parameter)
        .unwrap();

    if let Err(e) = writeln!(file, "\nResult of the command : {}",to_execute) {
        eprintln!("Couldn't write to file: {}", e);
    }
    if let Err(e) = writeln!(file, "\n{}",resultat) {
        eprintln!("Couldn't write to file: {}", e);
    }
}



fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    // Récupération du paramètre d'entrée
    let file_parameter = &args[1];

    // println!("Searching in {}", file_parameter);
    // println!("File output {}", file_parameter);

    bash(file_parameter.to_string());

    Ok(())
}
