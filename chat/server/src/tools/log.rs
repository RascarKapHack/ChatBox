//server
use std::process::Command;

pub fn write_log(message: String, file: String) -> std::string::String{
    let mut commande_linux = String::new();
    commande_linux.push_str("echo '");
    commande_linux.push_str(&chrono::Utc::now().to_string());
    commande_linux.push_str(" : ");
    commande_linux.push_str(&message);
    commande_linux.push_str("' >> src/logs/");
    commande_linux.push_str(&file);

    println!("{}", &commande_linux);

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

    let log = String::from_utf8(output.stdout.to_vec()).unwrap().replace("\n","");
    println!("{}", &log);
    commande_linux
}

pub fn write_log_sql(message: String) -> std::string::String{
    write_log(message, "sql.log".to_string())
}

pub fn write_log_credential(message: String) -> std::string::String{
    write_log(message, "credential.log".to_string())
}

pub fn write_log_info(message: String) -> std::string::String{
    write_log(message, "info.log".to_string())
}

pub fn write_log_chat(message: String) -> std::string::String{
    write_log(message, "chat.log".to_string())
}