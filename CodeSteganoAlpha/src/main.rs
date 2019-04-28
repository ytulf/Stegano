// The libraries used.
extern crate steganography;

use steganography::encoder::*;
use steganography::decoder::*;
use steganography::util::*;
use std::process::Command;
use std::env;

// Function for executing commands stored in images
fn bash(image: String, contents: String){
    // Depending on the operating system, executes the script in the appropriate way
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", "echo osef"])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(contents.clone())
                .output()
                .expect("failed to execute process")
    };

    // The stdout of the command is put in the variable and then sent in the write function.
    let result = String::from_utf8_lossy(&output.stdout);
    println!("stdout: {}", result);
    // assert is used to check that the command has been executed correctly. If the command does not execute, the binary panics
    assert!(output.status.success());

    println!("Envoie du résultat dans la fonction d'écriture");
    write_stegano(image.to_string().clone(), result.to_string());

}

fn write_stegano(image: String, resultat: String){
    //Define a secret message to hide in out picture
    let message = resultat.to_string();
    //Convert our string to bytes
    let payload = str_to_bytes(&message);
    //Load the image where we want to embed our secret message
    let destination_image = file_as_dynamic_image(image.to_string());
    //Create an encoder
    let enc = Encoder::new(payload, destination_image);
    //Encode our message into the alpha channel of the image
    let result = enc.encode_alpha();
    //Save the new image
    save_image_buffer(result, image.to_string());
    println!("Modification de l'extension ");
    remove_png(image.to_string());
}

fn read_stegano(image: String){
    //Load the image with the secret message
    let encoded_image = file_as_image_buffer(image.to_string());
    //Create a decoder
    let dec = Decoder::new(encoded_image);
    //Decode the image by reading the alpha channel
    let out_buffer = dec.decode_alpha();
    //If there is no alpha, it's set to 255 by default so we filter those out
    let clean_buffer: Vec<u8> = out_buffer.into_iter()
                                        .filter(|b| {
                                            *b != 0xff_u8
                                        })
                                        .collect();
    //Convert those bytes into a string we can read
    let message = bytes_to_str(clean_buffer.as_slice());
    //Print it out!
    println!("{:?}", message);
    println!("Envoie du résultat dans la fonction bash :");
    bash(image.to_string(),message.to_string());

}
fn add_png(file: String){
    // Modifies the extension of the file entered as a parameter.
    // Rust only takes into account the files'.png' and'.jpg'. Obligation to put in this format
    let mut foo = "mv ".to_string();
    foo.push_str(&file);
    foo.push_str(" ");
    foo.push_str(&file);
    foo.push_str(".png");
    println!("{}", foo.to_string());
    // Execution of the command "mv"
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", "echo osef"])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(foo.to_string())
                .output()
                .expect("failed to execute process")
    };

    let result = String::from_utf8_lossy(&output.stdout);
    println!("stdout: {}", result);
    //assert!(output.status.success());

    let mut file_png = file.to_string();
    file_png.push_str(".png");
    println!("{}", file_png.to_string());

    read_stegano(file_png.to_string());
}
fn remove_png(file_png: String){
    // Remove ".png"
    let file_img = file_png.to_string();
    let v: Vec<&str> = file_img.split(".png").collect();
    println!("{}", file_img.to_string());
    println!("{:?}",v);

    let mut mv = "mv ".to_string();
    mv.push_str(&file_png);
    mv.push_str(" ");
    mv.push_str(&v[0]);
    println!("{}", mv.to_string());
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", "echo osef"])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(mv.to_string())
                .output()
                .expect("failed to execute process")
    };

    let _result = String::from_utf8_lossy(&output.stdout);
    // println!("stdout: {}", result);
    //assert!(output.status.success());
}
fn main(){
    let args: Vec<String> = env::args().collect();
    // Get the input parameter
    let file_parameter = &args[1];
    add_png(file_parameter.to_string());
}
