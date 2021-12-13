pub mod sqlcommand;
pub mod log;
pub mod chiffrement;
pub mod design;

pub fn set_up_bdd(){
    sqlcommand::set_up_bdd();
}