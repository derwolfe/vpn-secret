extern crate gpgme;

use std::process::{Command, Stdio};
use std::io::Write;
use std::fs::File;

use gpgme::{Context, Protocol};

fn paste(target_text: Vec<u8>) -> String {
    let mut child = Command::new("pbcopy")
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to run pbcopy");
    {
        let stdin = child
            .stdin
            .as_mut()
            .expect("failed to get stdin for child");
        stdin
            .write_all(target_text.as_slice())
            .expect("failed to write to child's stdin");
    }

    let output = child
        .wait_with_output()
        .expect("failed waiting for child");
    String::from_utf8(output.stdout).expect("failed creating string from out")
}

// get the value from the file and return it
// this should return bytes instead
fn find(plaintext: Vec<u8>) -> String {
    let passwords = String::from_utf8(plaintext).expect("file isn't valid utf8");

    // split the string into lines, get the last one
    let target_line = passwords
        .split(" ")
        .last()
        .expect("failed splitting the file");

    // split the line and get the last element
    let password = target_line
        .split(" ")
        .last()
        .expect("failed getting password string");

    // allocate a new owned string that we can return out
    password.to_string()
}

fn decrypt_password_file() -> Vec<u8> {
    let input = File::open("/Users/Chris/Code/secure/rms/files/environment-vpn.gpg")
        .expect("couldn't open vpn secret file");
    let mut output = Vec::new();
    let mut ctx = Context::from_protocol(Protocol::OpenPgp).expect("failed building context");

    // why isn't this asking for a pin?
    ctx.decrypt(&input, &mut output)
        .expect("failed to decrypt file, need key");

    output
}

fn main() {
    let passwords = decrypt_password_file();
    let words = find(passwords);
    paste(words.into_bytes());
}

#[test]
fn decrypt_test() {
    let pw_string = decrypt_password_file();
    println!("{:?}", &pw_string);
}
