use std::env;
use std::process;

// 아래와 같이 하면 에러 발생하여서
// use minigrep::Config;

// https://stackoverflow.com/questions/53677463/rust-book-12-3-unresolved-import-error-e4032
// 참고하여 아래와 같이 import
mod lib;
use lib::Config;

/*
  다음 명령어로 실행해보기
  1. cargo run > output.txt
  2. cargo run to poem.txt > output.txt
*/
fn main() {
  let args: Vec<String> = env::args().collect();
  let config = Config::new(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  if let Err(e) = lib::run(config) {
    eprintln!("Application error: {}", e);

    process::exit(1);
  }
}
