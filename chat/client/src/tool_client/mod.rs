// client
pub mod design;

pub mod qr_code_client;

pub fn generate_qr_code_client(user: &str, passwd: &str){
    qr_code_client::qr_code_generator(user, passwd);
}
