// The libraries used.
extern crate steganography;
extern crate encryptfile;

// Modules used in libraries
use steganography::encoder::*;
use steganography::decoder::*;
use steganography::util::*;
use std::env;
use encryptfile as ef;

fn write_stegano(image: String, to_write: String){
    //Define a secret message to hide in out picture
    let message = to_write.to_string();
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
    //Print it out
    println!("{:?}", message);
}

fn encrypt(){
    let mut in_file = std::env::var("HOME").unwrap();
    in_file.push_str("/flag.txt");
    let mut c = ef::Config::new();
    c.input_stream(ef::InputStream::File(in_file.to_owned()))
     .output_stream(ef::OutputStream::File("/tmp/__encrypted.ef".to_owned()))
     .add_output_option(ef::OutputOption::AllowOverwrite)
     .initialization_vector(ef::InitializationVector::GenerateFromRng)
     .password(ef::PasswordType::Text("ipwnedyou".to_owned(), ef::scrypt_defaults()))
     .encrypt();
    let _ = ef::process(&c).map_err(|e| panic!("error encrypting: {:?}", e));
}

fn decrypt(){
    let mut c = ef::Config::new();
    c.input_stream(ef::InputStream::File("/tmp/__encrypted.ef".to_owned()))
     .output_stream(ef::OutputStream::File("/tmp/__encrypted.txt".to_owned()))
     .add_output_option(ef::OutputOption::AllowOverwrite)
     .password(ef::PasswordType::Text("ipwnedyou".to_owned(), ef::PasswordKeyGenMethod::ReadFromFile))
     .decrypt();
    let _ = ef::process(&c).map_err(|e| panic!("error decrypting: {:?}", e));
}
//
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    // Get the input parameter
    let file_parameter = &args[1];

    println!("Searching in {}", file_parameter);
    println!("File output {}", file_parameter);

    // If the -w option is filled in, then we write.
    if args.len() > 3 && &args[2] == "-w"{
        let message = &args[3];
        write_stegano(file_parameter.to_string(), message.to_string());
    }
    else{
        //Otherwise we just read the result
        read_stegano(file_parameter.to_string());
    }

    Ok(())
}
