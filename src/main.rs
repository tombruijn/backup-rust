use std::error::Error;
use std::str;
use std::fs::File;
use std::io::prelude::*;
use std::process::{Command, Stdio};

fn main() {
    // "echo 122 | cat > out"
    let file = "/Users/tombruijn/Downloads/dont_backup/FreeBSD-11.1-RELEASE-amd64-dvd1.iso";
    let process1 = match Command::new("tar")
        .args(&["-zcvf", "-", file])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
            Err(why) => panic!("couldn't spawn tar: {}", why.description()),
            Ok(process) => process,
        };

    let file = File::create("out.tar.gz").unwrap();
    let mut process2 = match Command::new("gzip")
        .stdin(process1.stdout.unwrap())
        .stdout(file)
        .spawn() {
            Err(why) => panic!("couldn't spawn tar: {}", why.description()),
            Ok(process) => process,
        };

    // let mut process3 = match Command::new("cat")
    //     .stdin(process2.stdout.unwrap())
    //     .stdout(file)
    //     .spawn() {
    //         Err(why) => panic!("couldn't spawn cat: {}", why.description()),
    //         Ok(process) => process,
    //     };

    process2.wait().expect("shit");
    // let mut buffer = Vec::new();
    // match process2.stdout.unwrap().read_to_end(&mut buffer) {
    //     Err(why) => panic!("couldn't read cat stdout: {}", why.description()),
    //     Ok(size) => {
    //         print!("cat responded with:\n{:?}", buffer);
    //         println!("foo {:?}", buffer);
    //         let mut file = File::create("out").unwrap();
    //         let mut arr = [0u8; 10];
    //         std::slice::bytes::copy_memory(&buffer, &mut arr);
    //         file.write(arr);
    //     },
    // }


    // let mut file = File::open("out").unwrap();
    // // let mut buffer = [0; 10];
    //
    // // read up to 10 bytes
    // // file.read(&mut buffer[..]).unwrap();
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).unwrap();
    // println!("read {:?}", contents);
    // // let s = match str::from_utf8(buffer) {
    // //     Ok(v) => v,
    // //     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    // // };
    // // println!("read: {:?}", s);
}
