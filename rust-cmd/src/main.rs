mod async_ls;
use std::process::Command;

#[tokio::main]
async fn main() {
    ls();
    async_ls::async_ls().await;
}

fn ls() {
    let output = Command::new("ls")
        .args(&["-l", "-a"])
        .output()
        .expect("failed to start `ls`");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
