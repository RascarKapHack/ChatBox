// server
extern crate sqlite;
extern crate colored;
use colored::*;

// Set up des tables
pub fn set_up_bdd(){
    let connection = sqlite::open("src/rust.db").unwrap();
    // creation bdd en ligne
    // Suppression de la base EN_LIGNE si elle existe
    connection
    .execute(
        "
        DROP TABLE IF EXISTS EN_LIGNE;
        "
    ).unwrap();
    println!("{}", "Réinitialise la table EN_LIGNE".italic().yellow());
    println!("{} \n{}", chrono::Utc::now().to_string().blue().bold(), "DROP TABLE IF EXISTS EN_LIGNE;".blue().bold());

    // Creation de la table EN_LIGNE si elle n'existe pas
    connection
    .execute(
        "
        CREATE TABLE IF NOT EXISTS EN_LIGNE (client_id TEXT, ip_port TEXT, hash TEXT);
        ",
    )
    .unwrap();
    println!("{}", "Crée la table EN_LIGNE si elle n'existe pas".italic().yellow());
    println!("{} \n{}", chrono::Utc::now().to_string().blue().bold(), "CREATE TABLE IF NOT EXISTS EN_LIGNE (client_id TEXT, ip_port TEXT, hash TEXT);".blue().bold());

    // Creation si la table INSCRITS n'existe pas
    connection
    .execute(
        "
        CREATE TABLE IF NOT EXISTS INSCRITS (identifiant TEXT, password TEXT, ip_port TEXT, en_ligne TEXT, match TEXT);
        ",
    )
    .unwrap();
    println!("{}", "Crée la table INSCRITS si elle n'existe pas".italic().yellow());
    println!("{} \n{}", chrono::Utc::now().to_string().blue().bold(), "CREATE TABLE IF NOT EXISTS INSCRITS (identifiant TEXT, password TEXT, ip_port TEXT, en_ligne TEXT, match TEXT);".blue().bold());
}