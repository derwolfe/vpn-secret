use std::process::{Command, Stdio};
use std::io::Write;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn paste(target_text: Vec<u8>) -> String {
    let mut child = Command::new("pbcopy")
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to run pbcopy");
    {
        let stdin = child.stdin.as_mut().expect("failed to get stdin for child");
        stdin.write_all(target_text.as_slice()).expect("failed to write to child's stdin");
    }

    let output = child
        .wait_with_output()
        .expect("failed waiting for child");
    String::from_utf8(output.stdout).expect("failed creating string from out")
}

// get the value from the file and return it
// this should return bytes instead
fn find() -> String {
    let file = File::open("/Users/Chris/Code/secure/rms/environment-vpn.txt")
        .expect("couldn't open vpn secrets files");
    let buf_reader = BufReader::new(file);

    let all_lines: Vec<String> = buf_reader
        .lines()
        .map(|line| { line.unwrap() })
        .collect();

    let target_line = all_lines
        .iter()
        .last()
        .unwrap();

    let password = target_line
        .split(" ")
        .last()
        .expect("couldn't get the final string");

    // allocate a new owned string that we can return out
    password.to_string()
}


fn main() {
    let words = find();
    paste(words.into_bytes());
}
