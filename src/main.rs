extern crate gpgme;
extern crate clap;

use std::process::{Command, Stdio};
use std::io::Write;
use std::fs::File;

use clap::{Arg, App};
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
    password
        .trim()
        .to_string()
}

fn decrypt_password_file(fp: String) -> Vec<u8> {
    let input = File::open(fp)
        .expect("couldn't open vpn secret file");
    let mut output = Vec::new();
    let mut ctx = Context::from_protocol(Protocol::OpenPgp)
        .expect("failed building context");
    ctx.decrypt(&input, &mut output)
        .expect("failed to decrypt file");

    output
}

fn parse_args() -> String {
    let matcher = App::new("VPN secret")
        .version("0.2.0")
        .about("fetches the last password in an encrypted file")
        .arg(Arg::with_name("encrypted-file")
             .short("f")
             .long("encryted-file")
             .value_name("FILE")
             .takes_value(true))
        .get_matches();
    matcher
        .value_of("encrypted-file")
        .unwrap()
        .to_string()
}

fn main() {
    let filepath = parse_args();
    let passwords = decrypt_password_file(filepath);
    let words = find(passwords);
    paste(words.into_bytes());
}

// #[test]
// fn decrypt_test() {
//     let pw_string = decrypt_password_file();
//     println!("{:?}", &pw_string);
// }
