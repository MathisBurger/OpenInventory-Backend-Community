use pwhash::sha512_crypt;

pub fn hash_including_salt(pwd: String) -> String {
    sha512_crypt::hash(pwd).expect("Could not hash password")
}