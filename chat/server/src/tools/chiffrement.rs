//server
use Command;

pub fn string_to_base32(secret: String) -> std::string::String{
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

// retourne la date UTC hashée
pub fn cle_chiffrement() -> std::string::String{
    let cle_chiffrement = chrono::Utc::today();
    sha256::digest(cle_chiffrement.to_string())
}