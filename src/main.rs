use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};

fn main() {
    // "echo 122 | cat > out"
    let process1 = match Command::new("echo")
        .arg("123")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
            Err(why) => panic!("couldn't spawn echo: {}", why.description()),
            Ok(process) => process,
        };
    let mut process2 = match Command::new("cat")
        // .arg("> out")
        .stdin(process1.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn() {
            Err(why) => panic!("couldn't spawn cat: {}", why.description()),
            Ok(process) => process,
        };

    process2.wait().expect("shit");
    let mut s = String::new();
    match process2.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read cat stdout: {}",
                           why.description()),
        Ok(_) => print!("cat responded with:\n{}", s),
    }
}
