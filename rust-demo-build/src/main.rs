use std::env;

pub mod shadow{
    include!(concat!(env!("OUT_DIR"), "/shadow.rs"));
}

fn main() {
    println!("{}",shadow::BRANCH); //master
    println!("{}",shadow::COMMIT_HASH);//8405e28e64080a09525a6cf1b07c22fcaf71a5c5
    println!("{}",shadow::SHORT_COMMIT);//8405e28e
    println!("{}",shadow::COMMIT_DATE);//2020-08-16T06:22:24+00:00
    println!("{}",shadow::COMMIT_AUTHOR);//baoyachi
    println!("{}",shadow::COMMIT_EMAIL);//xxx@gmail.com

    println!("{}",shadow::BUILD_OS);//macos-x86_64
    println!("{}",shadow::RUST_VERSION);//rustc 1.45.0 (5c1f21c3b 2020-07-13)
    println!("{}",shadow::RUST_CHANNEL);//stable-x86_64-apple-darwin (default)
    println!("{}",shadow::CARGO_VERSION);//cargo 1.45.0 (744bd1fbb 2020-06-15)
    println!("{}",shadow::PKG_VERSION);//0.3.13

    println!("{}",shadow::PROJECT_NAME);//shadow-rs
    println!("{}",shadow::BUILD_TIME);//2020-08-16 14:50:25
    println!("{}",shadow::BUILD_RUST_CHANNEL);//debug
}
