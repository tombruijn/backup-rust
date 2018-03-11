use std::error::Error;
use std::io;
use std::fs::File;
use std::process::{Command, Stdio};

fn main() {
    let file = "/Users/tombruijn/Downloads/dont_backup/FreeBSD-11.1-RELEASE-amd64-dvd1.iso";
    let process1 = match Command::new("tar")
        .args(&["-zcvf", "-", file])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .spawn() {
            Err(why) => panic!("couldn't spawn tar: {}", why.description()),
            Ok(process) => process,
        };

    let process2 = match Command::new("gzip")
        .stdin(process1.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn() {
            Err(why) => panic!("couldn't spawn tar: {}", why.description()),
            Ok(process) => {
                process
            },
        };

    let file = File::create("out.tar.gz").unwrap();
    let mut process3 = Command::new("cat");
    process3.stdin(Stdio::piped());
    process3.stdout(file);

    match process3.spawn() {
        Err(why) => panic!("couldn't spawn cat: {}", why.description()),
        Ok(process) => {
            let size = io::copy(&mut process2.stdout.unwrap(), &mut process.stdin.unwrap());
            println!("size: {:?}", size.unwrap());
        },
    };

    // match process3.stdin {
    //     Some(mut stdin) => {
    //         match process2.stdout {
    //             Some(mut out) => {
    //                 let size = io::copy(&mut out, &mut stdin);
    //                 println!("size: {:?}", size)
    //             },
    //             None => panic!("panic: {}")
    //         }
    //     },
    //     None => {}
    // };
    // process3.wait().expect("shit");

    // let stdout = match process2.stdout {
    //     None => panic!("foo {}"),
    //     Some(ref mut out) => {
    //         println!("foo: {:?}", out);
    //         let mut buffer = [0u8; 10];
    //         let length = match out.read(&mut buffer) {
    //             Ok(length) => length,
    //             Err(why) => panic!("error: {}", why),
    //         };
    //         println!("length: {:?}, content: {:?}", length, buffer);
    //     },
    // };
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
