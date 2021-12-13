extern crate qrcode;
use qrcode::QrCode;
use qrcode::render::unicode;
extern crate magic_crypt;
use magic_crypt::MagicCryptTrait;
extern crate colored;
use colored::*;
extern crate sha256;
use std::process::Command;

fn string_to_base32(secret: String) -> std::string::String{
    //let secret = "bouclefolle";
    let mut commande_linux = String::new();
    commande_linux.push_str("echo '");
    commande_linux.push_str(&secret);
    commande_linux.push_str("' | base32");
    //println!("{}", commande_linux);

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(["/C", &commande_linux])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(&commande_linux)
                .output()
                .expect("failed to execute process")
    };

    let base32_return = String::from_utf8(output.stdout.to_vec()).unwrap().replace("\n","");
    //println!("{}", hello);
    base32_return
}

pub fn qr_code_generator(user: &str, passwd: &str){

    //println!("{}", &user);
    //println!("{}", &passwd);
    
    let mut secret_fort = String::new();
    secret_fort.push_str(&user.replace("\n",""));
    secret_fort.push_str(&passwd.replace("\n",""));
    secret_fort=sha256::digest(secret_fort);
    //println!("{}", secret_fort);
    secret_fort=string_to_base32(secret_fort.to_string());
    
    //let secret_fort = "base32secret3232";

    //let stringdeouf = "otpauth://totp/".to_owned()+user+"?secret="+&passwd_encrypted+"&issuer=ChatBox";
    let stringdeouf = "otpauth://totp/".to_owned()+user+"?secret="+&secret_fort+"&issuer=ChatBox";
    let code = QrCode::new(stringdeouf).unwrap();
    let image = code.render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    println!("{}", "\nVoici votre QR Code, scannez le avec une application comme Google Authentificator.".red().bold());
    println!("{}", "\nCe token s'ajoute en complément de votre mot de passe lors de l'identification. \n".red().bold());
    println!("{}", image);
}