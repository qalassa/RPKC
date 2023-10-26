use rsa_lib::{keygen, encrypt, decrypt};

#[test]
fn rsa_end_to_end_test() {
    let (n, e, d) = keygen::generate_keypair(2048);
    let public_key = (n.clone(), e.clone());
    let private_key = (n, e, d);

    let message = b"Hello, RSA!";
    let encrypted = encrypt::encrypt(&public_key, message).unwrap();
    let decrypted = decrypt::decrypt(&private_key, &encrypted).unwrap();

    assert_eq!(message.to_vec(), decrypted);
}
