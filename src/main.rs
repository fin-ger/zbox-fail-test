use zbox;
use std::fs::File;
use std::io::Read;
use clap::{App, Arg, SubCommand};

fn main() {
    zbox::init_env();

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("file")
             .short("f")
             .long("file")
             .value_name("FILE")
             .help("specify the reference file for the test")
             .required(true))
        .subcommand(SubCommand::with_name("run")
                    .about("writes the content of the provided reference file to the zbox repo in chunks of 1MiB"))
        .subcommand(SubCommand::with_name("check")
                    .about("checks the contents of the zbox repo against the provided reference file"))
        .get_matches();

    let source_path = matches.value_of("file").unwrap();
    let mut source = File::open(source_path).unwrap();
    let mut repo = zbox::RepoOpener::new()
        .create(true)
        .open("file://./zbox-fail-test-repo", "zbox")
        .unwrap();

    let mut file = zbox::OpenOptions::new()
        .create(true)
        .open(&mut repo, "/test")
        .unwrap();

    if let Some(_) = matches.subcommand_matches("run") {
        // 1 MiB buffer
        let mut buffer = [0; 1024 * 1024];
        let mut pos = 0;

        // truncate old contents of file
        file.set_len(0).unwrap();

        loop {
            match source.read(&mut buffer) {
                Ok(length) => {
                    if length == 0 {

                        break;
                    }

                    println!("Writing chunk {} to repository ({}MiB)...", pos, (length as f32) / (1024.0 * 1024.0));
                    file.write_once(&buffer[0..length]).unwrap();
                    println!("Finished writing, new version created");
                },
                Err(err) => {
                    panic!(err);
                },
            };

            pos += 1;
        }
    } else if let Some(_) = matches.subcommand_matches("check") {
        // 1 MiB reference buffer
        let mut ref_buffer = [0; 1024 * 1024];
        let mut ref_len;
        // 1 MiB repo buffer
        let mut repo_buffer = [0; 1024 * 1024];
        let mut repo_len;
        let mut pos = 0;

        loop {
            match source.read(&mut ref_buffer) {
                Ok(length) => {
                    ref_len = length;
                },
                Err(err) => {
                    panic!(err);
                },
            };

            match file.read(&mut repo_buffer) {
                Ok(length) => {
                    repo_len = length;
                },
                Err(err) => {
                    panic!(err);
                },
            };

            if ref_len != repo_len {
                if repo_len > ref_len {
                    panic!("Repository contains more data than reference!");
                }
            }

            if repo_len == 0 {
                break;
            } else if repo_len != 1024 * 1024 {
                panic!("Incomplete chunk {} of size {}MiB", pos, (repo_len as f32) / (1024.0 * 1024.0));
            }

            if ref_buffer[..] != repo_buffer[..] {
                panic!("Found difference in chunk {}", pos);
            }

            println!("Chunk {} passed", pos);

            pos += 1;
        }
    } else {
        eprintln!("Please provide a subcommand! (run or check)");
    }
}
