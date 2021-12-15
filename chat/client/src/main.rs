// client

use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;
use std::process;

extern crate magic_crypt;
use magic_crypt::MagicCryptTrait;
//use magic_crypt::new_magic_crypt;

mod tool_client;
extern crate qrcode;
extern crate colored;
use colored::*;
extern crate sha256;

const LOCAL: &str = "192.168.1.63:6001";
const MSG_SIZE: usize = 128;

fn main() {
    /*
    struct Identification{
        user: String,
        password: String,
        choice: String,
    }*/

    let mut inscription = false;
    let mut input_choice = String::new();
    let mut input_user = String::new();
    let mut input_password = String::new();
    //let mut input_otp = String::new();

    //message de bienvenue
    tool_client::design::client();
    tool_client::design::welcome();

    // Inscription
    loop{
        if inscription == false{
            loop{
                println!("{}", "\n    Bienvenue sur le Chat. Souhaitez-vous vous inscrire ou vous connecter ?\n\n    1) S'inscrire\n\n    2) Se connecter\n\n".blue().bold());
                match io::stdin().read_line(& mut input_choice){
                    Ok(1) => {
                        if input_choice=="2\n"{
                            break;
                        }
                        if input_user.len() > 1{
                            println!("{}", input_choice);
                            break;
                        }
                    }
                    Ok(2) => {
                        let input_choice_without_n = input_choice.replace("\n","");
                        if input_choice_without_n == "1"{
                            input_choice="1\n".to_string();
                            break;
                        }
                        if input_choice_without_n == "2"{
                            input_choice="2\n".to_string();
                            break;
                        }
                        input_choice="".to_string();
                        println!("{:?}", input_choice_without_n);
                    }

                    Ok(_) => {
                        println!("{}","Here we go again :D".italic().magenta());
                    }
                    Err(e) => println!("oups {}", e)
                }
            }
            loop{
                if input_choice=="2\n"{
                    break;
                }
                println!("\nVeuillez entrer votre nom d'utilisateur");
                
                // CHECK SI LE PSEUDO EST DEJA PRIT
                match io::stdin().read_line(& mut input_user){
                    Ok(_) => {
                        if input_user.len() > 1{
                            //println!("...");

                            break;
                        }
                        input_user.clear();
                    }
                    Err(e) => println!("oups {}", e)
                }
                
            }
            loop{
                if input_choice=="2\n"{
                    break;
                }
                println!("\nVeuillez entrer votre mot de passe");

                match io::stdin().read_line(& mut input_password){
                    Ok(_) => {
                        if input_password.len() > 1{
                            input_password = sha256::digest(input_password);
                            let mut input_user2 = input_user.clone().replace("\n","");
                            let input_password_deux = input_password.clone().replace("\n","");
                            let input_choice_deux = input_choice.clone().replace("\n","");

                            // envoyer une demande au serveur et si retour positif ok
                            let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
                            client.set_nonblocking(true).expect("failed to initiate non-blocking");
                            let (tx, rx) = mpsc::channel::<String>();
                            
                            thread::spawn(move || loop {
                                let mut buff = vec![0; MSG_SIZE];
                                match client.read_exact(&mut buff) {
                                    Ok(_) => {
                                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                                        let msg = String::from_utf8(msg).expect("Invalid utf8 message");
                                        let mc = magic_crypt::new_magic_crypt!("cledeouf", 256);
                                        let message = mc.decrypt_base64_to_string(msg);

                                        match message {
                                            Ok(_) =>{
                                                //println!("{:?}", message.as_ref().unwrap());
                                                if message.as_ref().unwrap()=="true"{
                                                    println!("{}", "Votre compte à été créé avec succès.".green().bold());
                                                    println!("{}", "Appuyez sur Entrée pour générer le QR Code.".green().bold());
                                                    break;
                                                } else {
                                                    println!("{}", "Le pseudo est déjà existant.".red().bold());
                                                    println!("{}", "Bonne journée !".red().bold());
                                                    process::exit(0x0100);
                                                }
                                            }
                                            Err(_) => ()
                                        }
                                        
                                    },
                                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                                    Err(_) => {
                                        println!("la connection au server est coupée");
                                        break;
                                    }
                                }
                                
                                match rx.try_recv() {
                                    Ok(msg) => {
                                        let mut buff = msg.clone().into_bytes();
                                        buff.resize(MSG_SIZE, 0);
                                        client.write_all(&buff).expect("writing to socket failed");
                                    }, 
                                    Err(TryRecvError::Empty) => (),
                                    Err(TryRecvError::Disconnected) => break
                                }
                        
                                thread::sleep(Duration::from_millis(100));
                            });
                            
                            //let ararara: [String; 2] = [String::from(input_user2.to_string()+"\n\t\n"+&input_password_deux.to_string()),String::from("inscription").to_string()];
                            //let ararara: [String; 3] = [String::from(input_user2.to_string().replace("\n","")), String::from(input_password_deux.to_string().replace("\n","")),String::from("inscription").to_string()];
                            //let envoi_informations = 
                            //println!("{:?}", ararara);
                            
                            loop {
                                let buff2;
                                input_user2.push_str("-");
                                input_user2.push_str(&input_password_deux);
                                input_user2.push_str("-");
                                input_user2.push_str(&input_choice_deux);
                                input_user2.push_str("-");
                                input_user2.push_str("INSCRIPTION");

                                //println!("{}", input_user2);
                                buff2 = input_user2.to_string();
                                //buff2 = ararara[0].to_string();
                                let msg = buff2.trim().to_string();

                                let mc = magic_crypt::new_magic_crypt!("cledeouf", 256);
                                let base64_msg = mc.encrypt_str_to_base64(msg);

                                if tx.send(base64_msg).is_err() {break}
                                //if msg == ":quit" || tx.send(msg).is_err() {break}
                                break;
                            }

                            // Lancement du programme si retour ok du serveur
                            loop {
                                let mut buff = String::new();
                                io::stdin().read_line(&mut buff).expect("reading from stdin failed");
                                let msg = buff.trim().to_string();
                                let msg2 = msg.clone();
                                let mc = magic_crypt::new_magic_crypt!("cledeouf", 256);
                                let base64_msg = mc.encrypt_str_to_base64(msg2);
                                //println!("{}",base64_msg);
                                //println!("{}", mc.decrypt("base64"));
                
                                if msg == ":quit" || tx.send(base64_msg).is_err() {break}
                            }


                            break;
                        }
                        input_password.clear();
                    }
                    Err(e) => println!("oups {}", e)
                }

            }
            if input_choice=="1\n"{
                //envoie du QR Code
                tool_client::generate_qr_code_client(&input_user.replace("\n",""), &input_password);
                inscription = true;
            }
            if input_choice=="2\n"{
                inscription = true;
            }
        }
        else{
            break;
        }
    }

    //let mut input_choice = String::new();
    let mut input_user = String::new();
    let mut input_password = String::new();
    let mut input_otp = String::new();

    loop{
        println!("\nVeuillez entrer votre nom d'utilisateur");
        match io::stdin().read_line(& mut input_user){
            Ok(_) => {
                if input_user.len() > 1{
                    break;
                }
                input_user.clear();
            }
            Err(e) => println!("oups {}", e)
        }
    }

    loop{
        println!("\nVeuillez entrer votre mot de passe");
        match io::stdin().read_line(& mut input_password){
            Ok(_) => {
                if input_password.len() > 1{
                    input_password=sha256::digest(input_password);
                    break;
                }
                input_password.clear();
            }
            Err(e) => println!("oups {}", e)
        }
    }

    loop{
        println!("\nVeuillez entrer votre OTP");
        match io::stdin().read_line(& mut input_otp){
            Ok(_) => {
                if input_otp.len() > 1{

                    let mut input_user2 = input_user.clone().replace("\n","");
                    let input_password_deux = input_password.clone().replace("\n","");
                    //let input_choice_deux = input_choice.clone().replace("\n","");
                    let input_otp_deux = input_otp.clone().replace("\n","");

                    // envoyer une demande au serveur et si retour positif ok
                    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
                    client.set_nonblocking(true).expect("failed to initiate non-blocking");
                    let (tx, rx) = mpsc::channel::<String>();
                    
                    thread::spawn(move || loop {
                        let mut buff = vec![0; MSG_SIZE];
                        match client.read_exact(&mut buff) {
                            Ok(_) => {
                                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                                let msg = String::from_utf8(msg).expect("Invalid utf8 message");
                                let mc = magic_crypt::new_magic_crypt!("cledeouf", 256);
                                let message = mc.decrypt_base64_to_string(msg);

                                match message {
                                    Ok(_) =>{
                                        //println!("{:?}", message.as_ref().unwrap());
                                        if message.as_ref().unwrap()=="false"{
                                            println!("{}", "Successful, vous vous êtes bien identifié.".green().bold());
                                            println!("{}", "Double appuyer sur Entrée pour continuer.".green().bold());
                                            break;
                                        } else {
                                            println!("{}", "Votre identification à échoué, veuillez revoir vos identifiants..".red().bold());
                                            println!("{}", "Bonne journée !".red().bold());
                                            process::exit(0x0100);
                                        }
                                    }
                                    Err(_) => ()
                                }
                                
                            },
                            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                            Err(_) => {
                                println!("la connection au server est coupée");
                                break;
                            }
                        }
                        
                        match rx.try_recv() {
                            Ok(msg) => {
                                let mut buff = msg.clone().into_bytes();
                                buff.resize(MSG_SIZE, 0);
                                client.write_all(&buff).expect("writing to socket failed");
                            }, 
                            Err(TryRecvError::Empty) => (),
                            Err(TryRecvError::Disconnected) => break
                        }
                
                        thread::sleep(Duration::from_millis(100));
                    });
                                        
                    loop {
                        let buff2;
                        input_user2.push_str("-");
                        input_user2.push_str(&input_password_deux);
                        input_user2.push_str("-");
                        input_user2.push_str("2");
                        input_user2.push_str("-");
                        input_user2.push_str(&input_otp_deux);

                        //println!("{}", input_user2);
                        buff2 = input_user2.to_string();
                        //buff2 = ararara[0].to_string();
                        let msg = buff2.trim().to_string();

                        let mc = magic_crypt::new_magic_crypt!("cledeouf", 256);
                        let base64_msg = mc.encrypt_str_to_base64(msg);

                        if tx.send(base64_msg).is_err() {break}
                        //if msg == ":quit" || tx.send(msg).is_err() {break}
                        break;
                    }

                    // Lancement du programme si retour ok du serveur
                    loop {
                        let mut buff = String::new();
                        io::stdin().read_line(&mut buff).expect("reading from stdin failed");
                        let msg = buff.trim().to_string();
                        let msg2 = msg.clone();
                        let mc = magic_crypt::new_magic_crypt!("cledeouf", 256);
                        let base64_msg = mc.encrypt_str_to_base64(msg2);
                        //println!("{}",base64_msg);
                        //println!("{}", mc.decrypt("base64"));
        
                        if msg == ":quit" || tx.send(base64_msg).is_err() {break}
                    }


                    break;

                }
                input_otp.clear();
            }
            Err(e) => println!("oups {}", e)
        }
    }


    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    /*let info = Identification {
        user: String::from(input_user),
        password: String::from(input_password),
        choice: String::from(input_choice),
        //otp: String::from(input_otp),
    };*/

    //println!("{:?}", client);
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client.set_nonblocking(true).expect("failed to initiate non-blocking");
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                let msg = String::from_utf8(msg).expect("Invalid utf8 message");
                let now = chrono::Utc::now();
                let mc = magic_crypt::new_magic_crypt!("cledeouf", 256);
                let message = mc.decrypt_base64_to_string(msg);

                
                //println!("{} {} {}", info.user.to_string().replace("\n",""), info.password.to_string().replace("\n",""), info.choice.to_string().replace("\n",""));

                match message {
                    Ok(_) =>{
                        println!("[{}] - {}", now.format("%b %-d, %-I:%M").to_string().yellow().bold(), message.unwrap());
                    }
                    Err(_) => ()
                }

            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("la connection au server est coupée");
                break;
            }
        }

        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("writing to socket failed");
            }, 
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }

        thread::sleep(Duration::from_millis(100));
    });
            
    // Envoie les informations de connection au serveur. Panique si les informations ne sont pas bonnes. 

    // ne sert a rien
    /*
    loop {
        let buff2;
        
        let mut credentials = String::new();
        credentials.push_str("batlesta");
        credentials.push_str("-");
        credentials.push_str("&info.password");
        credentials.push_str("-");
        credentials.push_str("3");
        credentials.push_str("-");
        credentials.push_str("R");

        //println!("{}", input_user2);
        buff2 = credentials.to_string();
        //buff2 = ararara[0].to_string();
        let msg = buff2.trim().to_string();

        //let mc = magic_crypt::new_magic_crypt!("cledeouf", 256);
        //let base64_msg = mc.encrypt_str_to_base64(msg);

        break;
    }*/
    
    //
    // Lancement du programme si retour ok du serveur

    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("reading from stdin failed");
        let msg = buff.trim().to_string();
        let msg2 = msg.clone();
        let mc = magic_crypt::new_magic_crypt!("cledeouf", 256);
        let base64_msg = mc.encrypt_str_to_base64(msg2);
        //println!("{}",base64_msg);
        
        //println!("{}", mc.decrypt("base64"));

        //println!("{}", msg);
        if msg == ":quit" || tx.send(base64_msg).is_err() {break}
    }
    println!("bye bye!");
}

// -------------------------------- TEST -----------------------------------------

pub fn add(a: i32, b:i32) -> i32{
    a + b
}

#[cfg(test)]
mod tests {
    use super ::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1,2),3);
    }
}
