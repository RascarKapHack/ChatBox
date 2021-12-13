// server

use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
extern crate sqlite;
extern crate magic_crypt;
use magic_crypt::MagicCryptTrait;
use std::process::Command;
extern crate sha256;
extern crate colored;
use colored::*;
extern crate google_authenticator;
mod tools;

const LOCAL: &str = "192.168.1.63:6001";
const MSG_SIZE: usize = 128;

fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100));
}

fn main() {

    let cle_chiffrement_master = tools::chiffrement::cle_chiffrement();
    println!("La clé de chiffrement : {}", cle_chiffrement_master);
    tools::log::write_log_info("Nouvelle connexion".to_string());

    tools::design::affiche_serveur();

    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    server.set_nonblocking(true).expect("failed to initialize non-blocking");
    let mut inscription_ou_connexion = false;
    let mut premiereconnection = true;
    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<String>();
    let mut number: u8 = 1;

    // Set up des tables de la bdd
    tools::set_up_bdd();

    loop {

        if let Ok((mut socket, addr)) = server.accept() {
            let connection_du_client = "Le client ".to_owned() + &addr.to_string() + " s'est connecté";
            println!("{}", connection_du_client);
            tools::log::write_log_credential(connection_du_client.to_string());
            //println!("{}", number);

            let addresse_ip = &addr.to_string();
            //println!("{}", addresse_ip);
            // Mets a jour la table En_Ligne
            
            let mut sqlcommande = String::from("INSERT INTO EN_LIGNE VALUES (");
            sqlcommande.push_str("'Client ");
            sqlcommande.push_str(&number.to_string());
            sqlcommande.push_str("','");
            sqlcommande.push_str(&addresse_ip.to_string());
            sqlcommande.push_str("','anonyme');");
            println!("{} {}", chrono::Utc::now(), sqlcommande.cyan());
            number+=1;
            let connection = sqlite::open("src/rust.db").unwrap();
            connection
                .execute(
                    sqlcommande
                    ,
                )
                .unwrap();
            
            let tx = tx.clone();
            clients.push(socket.try_clone().expect("failed to clone client"));
            
            thread::spawn(move || loop {
                let mut buff = vec![0; MSG_SIZE];
                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        //println!("tetetete {}", msg);
                        // Checker en base de données et integrer les informations

                        if inscription_ou_connexion==false{
                            let mut vecteurmsg = Vec::new();
                            let mut tableau_info = Vec::new();
                            let msg2 = msg.clone();

                            let connection1 = sqlite::open("src/rust.db").unwrap();

                            //println!("{:?}", msg2);
                            // convertir unicode en message
                            for x in msg2{
                                let min:u8 = 10;
                                let mun:u8 = 9;
                                if x!=min && x!=mun{
                                    vecteurmsg.push(x)
                                }else{
                                    tableau_info.push(vecteurmsg);
                                    vecteurmsg = Vec::new();
                                }
                            }
                            let msg_clair = String::from_utf8(vecteurmsg.to_vec()).unwrap();
                            let msg_clair2 = msg_clair.clone();
                            let mut utilisateur_connu = false;
                            
                            //println!("{}", msg_clair2);
                            let mc = magic_crypt::new_magic_crypt!("cledeouf", 256);
                            let base64_msg = mc.decrypt_base64_to_string(msg_clair2);

                            match base64_msg {
                                Ok(_) =>{
                                    let msg_clair = base64_msg.as_ref().unwrap();
                                    let split = msg_clair.split("-");
                                    let vec: Vec<&str> = split.collect();
                                    //println!("{:?}", vec);
                                    //println!("{:?}", vec[0]);
                                    // chat
                                    if vec[0] == ""{
                                        //println!("eeee");

                                        // Mets a jour la table EN_LIGNE 
                                        let connection4 = sqlite::open("src/rust.db").unwrap();
                                        let mut sqlcommande4 = String::from("UPDATE EN_LIGNE SET hash=\"inscrit\" WHERE ip_port=");
                                        sqlcommande4.push_str("\"");
                                        sqlcommande4.push_str(&addr.to_string());
                                        sqlcommande4.push_str("\"");
                                        println!("{}", "Mets a jour la table EN_LIGNE => passe le statut en inscrit pour la bonne addresse ip:port".yellow().italic());
                                        println!("{}", sqlcommande4.cyan());

                                        connection4
                                        .execute(
                                            sqlcommande4
                                            ,
                                        )
                                        .unwrap();

                                        //let mut sqlcommande4_2 = String::from("SELECT identifiant FROM INSCRITS WHERE match='ko' AND en_ligne='ok'");
                                        let mut identifiant = String::new();
                                        connection1
                                            .iterate("SELECT identifiant FROM INSCRITS WHERE match=\"ko\" AND en_ligne=\"ok\"", |pairs| {
                                                for &(_column, value) in pairs.iter() {
                                                    //println!("{} = {}", column, value.unwrap());
                                                    //let test = column;
                                                    identifiant.push_str(value.unwrap());
                                                }
                                                true
                                            })
                                            .unwrap();
                                        //println!("{}", identifiant);
                                        
                                        // INSCRITS
                                        let sqlcommandemajinscrits = "UPDATE INSCRITS SET match=\"ok\", ip_port='".to_owned()+&addr.to_string()+"' WHERE identifiant='"+&identifiant+"'";
                                        let sqlcommandemajenligne = "UPDATE EN_LIGNE SET client_id='".to_owned()+&identifiant+"', hash=\"en_ligne\" WHERE ip_port='"+&addr.to_string()+"'";
                                        println!("{}","Mets a jour la table INSCRITS => ajoute ok si la personne est connectée".yellow().italic());
                                        println!("{}", sqlcommandemajinscrits.cyan());
                                        println!("{}","Mets a jour la table EN_LIGNE => Si la personne est inscrite ajoute son pseudo et passe le statut d'inscrit à en_ligne".yellow().italic());
                                        println!("{}", sqlcommandemajenligne.cyan());

                                        connection4
                                        .execute(
                                            sqlcommandemajinscrits
                                            ,
                                        )
                                        .unwrap();
                                        connection4
                                        .execute(
                                            sqlcommandemajenligne
                                            ,
                                        )
                                        .unwrap();
                                        
                                        //println!("{}", addr);
                                        let mut message_bienvenue = String::new();
                                        message_bienvenue.push_str("CHATBOX : Bienvenue sur le chat ");
                                        message_bienvenue.push_str(&identifiant);

                                        let mc = magic_crypt::new_magic_crypt!("cledeouf", 256);
                                        let base64_msg = mc.encrypt_str_to_base64(message_bienvenue);
                                        tx.send(String::from(base64_msg)).expect("failed to send msg to rx");
                                        //inscription_ou_connexion=true;

                                    }else{
                                        let user_id = vec[0];
                                        let user_passwd = vec[1];
                                        let user_choice = vec[2];
                                        let user_otp = vec[3];
                                        // inscription
                                        if user_choice=="1"{
                                            //println!("{}", msg_clair);
                                            connection1
                                            .iterate("SELECT * FROM INSCRITS WHERE identifiant = '".to_owned()+&user_id.to_string()+"';", |pairs| {
                                                for &(column,value) in pairs.iter(){
                                                    println!("{} {} {}", column.to_string().black().hidden(), "=".black().hidden(), value.unwrap().to_string().black().hidden());
                                                    //println!("l'utilisateur est connu");
                                                    utilisateur_connu = true;
                                                    break;
                                                }
                                                true
                                            })
                                            .unwrap();
                                            println!("{}", "Requête dans la table INSCRITS si la personne est inscrite".italic().yellow());
                                            println!("{} \n{} {}", chrono::Utc::now().to_string().blue().bold(),"SELECT * FROM INSCRITS WHERE identifiant = '".to_owned().bold().blue(), &user_id.to_string().bold().blue());
                                            tools::log::write_log_sql("SELECT * FROM INSCRITS WHERE identifiant = '".to_owned()+&user_id.to_string());
                                            if utilisateur_connu{
                                                println!("{} {} {}", "l'utilisateur".yellow().italic(), user_id.to_string().yellow().italic(), "est déjà inscrit(e) dans la bdd.".yellow().italic());
                                            }
                                            else{
                                                println!("{} {} {}", "l'utilisateur".yellow().italic(), user_id.to_string().yellow().italic(), "n'est pas inscrit(e). Et peut être ajouté à la base de donnée.".yellow().italic());
                                                
                                                connection1
                                                    .execute(
                                                        "
                                                        INSERT INTO INSCRITS VALUES ('".to_owned()+&user_id+"', '"+&user_passwd+"', '"+&addr.to_string()+"', 'ko', 'ko');
                                                        ",
                                                    )
                                                    .unwrap();
                                                println!("{}", "Ajout de la personne dans la base de donnée INSCRITS".yellow().italic());
                                                println!("{} \n{} {} {} {} {} {} {}", chrono::Utc::now().to_string().blue().bold(), "INSERT INTO INSCRITS VALUES ('".blue().bold(), &user_id.blue().bold(), "', '".blue().bold(), &user_passwd.blue().bold(), "', '".blue().bold(), &addr.to_string().blue().bold(), "', 'ko', 'ko');".blue().bold());
                                                tools::log::write_log_sql("INSERT INTO INSCRITS VALUES ('".to_owned()+&user_id.to_string()+ "," +&user_passwd + "," + &addr.to_string() + "', 'ko', 'ko'");
                                                println!("{}", "le compte à bien été créé".green().bold())
                                            }
                                            //println!("danaza {}", addr);

                                            // Réponse au client et lui envoie une balise de deconnexion si son pseudo était déjà prit.
                                            if utilisateur_connu{
                                                let autorisation = "false";
                                                let mc = magic_crypt::new_magic_crypt!("cledeouf", 256);
                                                let base64_msg = mc.encrypt_str_to_base64(autorisation);
                                                tx.send(String::from(base64_msg)).expect("failed to send msg to rx");
                                            }
                                            else{
                                                let autorisation = "true";
                                                let mc = magic_crypt::new_magic_crypt!("cledeouf", 256);
                                                let base64_msg = mc.encrypt_str_to_base64(autorisation);
                                                tx.send(String::from(base64_msg)).expect("failed to send msg to rx");
                                            }
    
                                        }
                                        // connection
                                        else if user_choice=="2"{
                                            //println!("step 3");
                                            utilisateur_connu = false;
                                            let mut password_correct = false;
                                            //println!("step 4 {}", addr);
                                            // Check en base de l'identifiant
                                            connection1
                                            .iterate("SELECT * FROM INSCRITS WHERE identifiant = '".to_owned()+&user_id.to_string()+"';", |pairs| {
                                                for &(column,value) in pairs.iter(){
                                                    println!("{} {} {}", column.to_string().hidden().black(), "=".hidden().black(), value.unwrap().to_string().hidden().black());
                                                    //println!("l'utilisateur est connu");
                                                    utilisateur_connu = true;
                                                    break;
                                                }
                                                true
                                            })
                                            .unwrap();
                                            // Check en base du password
                                            println!("{}", "Requête dans la table INSCRITS si la personne est inscrite".italic().yellow());
                                            println!("{} \n{} {}", chrono::Utc::now().to_string().blue().bold(), "SELECT * FROM INSCRITS WHERE identifiant = '".to_owned().bold().blue(), &user_id.to_string().bold().blue());
                                            tools::log::write_log_sql("SELECT * FROM INSCRITS WHERE identifiant = '".to_owned()+&user_id.to_string());

                                            connection1
                                            .iterate("SELECT * FROM INSCRITS WHERE password = '".to_owned()+&user_passwd.to_string()+"';", |pairs| {
                                                for &(column,value) in pairs.iter(){
                                                    println!("{} {} {}", column.to_string().hidden().black(), "=".hidden().black(), value.unwrap().to_string().hidden().black());
                                                    //println!("l'utilisateur est connu");
                                                    password_correct = true;
                                                    break;
                                                }
                                                true
                                            })
                                            .unwrap();
                                            println!("{}", "Requête dans la table INSCRITS si le mot de passe est valide".italic().yellow());
                                            println!("{} \n{} {}", chrono::Utc::now().to_string().blue().bold(), "SELECT * FROM INSCRITS WHERE password = '".to_owned().bold().blue(), &user_passwd.to_string().bold().blue());
                                            tools::log::write_log_sql("SELECT * FROM INSCRITS WHERE password = '".to_owned()+&user_passwd.to_string());

                                            let mut secret_fort = String::new();
                                            secret_fort.push_str(&user_id.replace("\n",""));
                                            secret_fort.push_str(&user_passwd.replace("\n",""));
                                            secret_fort=sha256::digest(secret_fort);
                                            secret_fort=tools::chiffrement::string_to_base32(secret_fort.to_string());
                                            let totp = otp::make_totp(&(secret_fort.to_ascii_uppercase()), 30, 0).unwrap().to_string();

                                            // suppression du drapeau anonyme
                                            let mut sqlcommande4 = String::from("DELETE FROM EN_LIGNE WHERE ip_port=");
                                            sqlcommande4.push_str("\"");
                                            sqlcommande4.push_str(&addr.to_string());
                                            sqlcommande4.push_str("\"");
                                            println!("{}", sqlcommande4.cyan());
                                            connection1
                                            .execute(
                                                sqlcommande4
                                                ,
                                            )
                                            .unwrap();
                                            println!("{}", "Suppression du drapeau anonyme de la table EN_LIGNE".italic().yellow());
                                            println!("{} \n{} {} {}", chrono::Utc::now().to_string().blue().bold(), "DELETE FROM EN_LIGNE WHERE ip_port='".bold().blue(), &addr.to_string().bold().blue(), "'".bold().blue());
                                            tools::log::write_log_sql("DELETE FROM EN_LIGNE WHERE ip_port='".to_owned()+&addr.to_string());

                                            if totp.to_string()==user_otp && utilisateur_connu==true && password_correct==true{
                                                println!("{}", "Identifiants valides. Envoie une autorisation de se connecter au Serveur".green().bold());

                                                let connection6 = sqlite::open("src/rust.db").unwrap();
                                                let mut sqlcommande6 = String::from("UPDATE INSCRITS SET en_ligne=\"ok\" WHERE identifiant=");
                                                sqlcommande6.push_str("\"");
                                                sqlcommande6.push_str(&user_id.to_string());
                                                sqlcommande6.push_str("\"");
                                                println!("{}", "Mets à jour la table INSCRITS => attribue ok au champ en ligne".yellow().italic());
                                                println!("{}", &sqlcommande6.cyan());
                                                connection6
                                                    .execute(
                                                        sqlcommande6
                                                        ,
                                                    )
                                                    .unwrap();


                                                let autorisation = "false";
                                                let mc = magic_crypt::new_magic_crypt!("cledeouf", 256);
                                                let base64_msg = mc.encrypt_str_to_base64(autorisation);
                                                tx.send(String::from(base64_msg)).expect("failed to send msg to rx");
                                            }
                                            else{
                                                println!("{}", "La connexion à été refusée".red().bold());
                                                if utilisateur_connu==false{
                                                    println!("{}", "Un utilisateur non inscrit à essayé de se connecter".bold().red());
                                                }
                                                if totp.to_string()==user_otp{
                                                    println!("{}", "L'utilisateur à renseigné un OTP non valide".bold().red());
                                                }
                                                let autorisation = "true";
                                                let mc = magic_crypt::new_magic_crypt!("cledeouf", 256);
                                                let base64_msg = mc.encrypt_str_to_base64(autorisation);
                                                tx.send(String::from(base64_msg)).expect("failed to send msg to rx");
                                            }
                                            //println!("{}", addr);
                                            break;
                                        }
                                        //println!("ici {}", addr);
                                    }
                                    inscription_ou_connexion=true;
                                    //println!("step1 {}", addr);
                                }
                                Err(_) => ()
                            }
                        }
                        //println!("step2 {}", addr);

                        if premiereconnection==false{
                            let mut msgwithexpediteur = String::new();
                            let msg = String::from_utf8(msg).expect("Invalid utf8 message");

                            
                            let connection7 = sqlite::open("src/rust.db").unwrap();
                            let mut identifiant = String::new();
                            connection7
                                .iterate("SELECT client_id FROM EN_LIGNE WHERE ip_port='".to_owned()+&addr.to_string()+"'", |pairs| {
                                    for &(_column, value) in pairs.iter() {
                                        //println!("{} = {}", column, value.unwrap());
                                        //let test = column;
                                        identifiant.push_str(value.unwrap());
                                    }
                                    true
                                })
                                .unwrap();
                            println!("{}", "Selectionne le bon identifiant à mettre pour connaitre l'expediteur du message en en-tête".yellow().italic());
                            println!("{} {} {}", "SELECT client_id FROM EN_LIGNE WHERE ip_port='".cyan(), &addr.to_string().cyan(), "'".cyan());

                            println!("{:?} - {} - {}", addr, identifiant, msg);
                            tools::log::write_log_chat(msg.to_string());

                            let mc = magic_crypt::new_magic_crypt!("cledeouf", 256);
                            let base64_msg = mc.decrypt_base64_to_string(msg).unwrap();

                            msgwithexpediteur.push_str(&identifiant);
                            msgwithexpediteur.push_str(" - ");
                            msgwithexpediteur.push_str(&base64_msg);
                            
                            let msg = mc.encrypt_str_to_base64(msgwithexpediteur);
                            
                            // ici ajouter le user de la bdd via jointure en ligne -- user
                            tx.send(msg).expect("failed to send msg to rx");
                            
                        }
                        premiereconnection=false;
                        
                    }, 
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("{} {} {}", "Le client".red().bold(),addr.to_string().red().bold(), "s'est deconnecté".red().bold());

                        // Suppression de la table EN_LIGNE
                        let connection3 = sqlite::open("src/rust.db").unwrap();

                        let mut sqlcommande3 = String::from("DELETE FROM EN_LIGNE WHERE ip_port=");
                        sqlcommande3.push_str("\"");
                        sqlcommande3.push_str(&addr.to_string());
                        sqlcommande3.push_str("\"");
                        println!("{}", "Suppression de la table EN_LIGNE de l'utilisateur.".yellow().italic());
                        println!("{}", sqlcommande3.cyan());

                        connection3
                        .execute(
                            sqlcommande3,
                        )
                        .unwrap();

                        connection3
                        .execute(
                            "UPDATE INSCRITS SET en_ligne='ko', match='ko' WHERE ip_port='".to_owned()+&addr.to_string()+"'"
                        )
                        .unwrap();
                        println!("{}", "Mets à jour la table INSCRITS et redefini les champs en_ligne et match par default.".yellow().italic());
                        println!("{} {}","UPDATE INSCRITS SET en_ligne='ko', match='ko' WHERE ip_port=".cyan(), &addr.to_string().cyan());

                        break;
                    }
                }

                sleep();
            });
        }
        
        if let Ok(msg) = rx.try_recv() {
            clients = clients.into_iter().filter_map(|mut client| {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).map(|_| client).ok()
            }).collect::<Vec<_>>();
        }

        sleep();
    }
}


#[cfg(test)]
mod chiffrement {
    use super ::*;
    #[test]
    fn string_to_base32() {
        assert_eq!(tools::chiffrement::string_to_base32("testdelafonction".to_string()),"ORSXG5DEMVWGCZTPNZRXI2LPNYFA====");
    }
    #[test]
    fn cle_chiffrement(){
        assert_eq!(tools::chiffrement::cle_chiffrement(),sha256::digest(chrono::Utc::today().to_string()))
    }
}

#[cfg(test)]
mod log{
    use super ::*;
    #[test]
    fn write_log_sql() {
        assert_eq!(tools::log::write_log_sql("Message".to_string()), "echo 'Message' >> src/logs/sql.log");
    }
    #[test]
    fn write_log_credentials() {
        assert_eq!(tools::log::write_log_credential("Message".to_string()), "echo 'Message' >> src/logs/credential.log");
    }
    #[test]
    fn write_log_info() {
        assert_eq!(tools::log::write_log_info("Message".to_string()), "echo 'Message' >> src/logs/info.log");
    }
    #[test]
    fn write_log_chat() {
        assert_eq!(tools::log::write_log_chat("Message".to_string()), "echo 'Message' >> src/logs/chat.log");
    }
    #[test]
    fn affiche_serveur() {
        assert_eq!(tools::design::affiche_serveur(), ());
    }
}