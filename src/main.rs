use std::error::Error;
use std::io;
use std::fs::File;
use std::process::{Command, Stdio};

fn main() {
    let file = "/Users/tombruijn/Downloads/dont_backup/FreeBSD-11.1-RELEASE-amd64-dvd1.iso";

    let archiver = archive(&file);
    let compressor = compress(archiver.stdout.unwrap());
    let stored_size = store("out.tar.gz", compressor.stdout.unwrap());
    println!("stored backup size: {:?}", stored_size);
}

fn archive(file: &str) -> std::process::Child {
    let mut command = Command::new("tar");
    command.args(&["-zcvf", "-", file]);
    command.stdin(Stdio::null());
    command.stdout(Stdio::piped());
    return match command.spawn() {
        Err(why) => panic!("couldn't spawn tar: {}", why.description()),
        Ok(process) => process,
    };
}

fn compress(stream: std::process::ChildStdout) -> std::process::Child {
    let mut command = Command::new("gzip");
    command.stdin(stream);
    command.stdout(Stdio::piped());
    return match command.spawn() {
        Err(why) => panic!("couldn't spawn gzip: {}", why.description()),
        Ok(process) => process,
    };
}

// Only for testing purposes. The last process in the chain can store it. No need for `cat` to do
// that. Only useful for the local file storage.
fn store(file: &str, mut stream: std::process::ChildStdout) -> u64 {
    let file = File::create(file).unwrap();
    let mut command = Command::new("cat");
    command.stdin(Stdio::piped());
    command.stdout(file);
    let process = match command.spawn() {
        Err(why) => panic!("couldn't spawn cat: {}", why.description()),
        Ok(process) => process,
    };

    match process.stdin {
        Some(mut stdin) => {
            match io::copy(&mut stream, &mut stdin) {
                Ok(size) => return size,
                Err(why) => panic!("error {}", why)
            }
        },
        None => panic!("foo")
    }
}
