use orion::*;

assert_eq!(format!("The origin is: {}", origin), "The origin is: (0, 0)");

pub fn encrypt(){
    let secret_key = aead::SecretKey::default();
    let ciphertext = aead::seal(&secret_key, "Secret message".as_bytes());
}

fn decrypt(){

}