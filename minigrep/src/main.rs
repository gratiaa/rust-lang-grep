use std::env;
use std::process;

// 아래와 같이 하면 에러 발생하여서
// use minigrep::Config;

// https://stackoverflow.com/questions/53677463/rust-book-12-3-unresolved-import-error-e4032
// 참고하여 아래와 같이 import
mod lib;
use lib::Config;

// 다음 명령어로 실행해보기: export CASE_INSENSITIVE=1; cargo run to poem.txt
// 설정된 환경 변수 확인: echo $CASE_INSENSITIVE
// 환경 변수 shell에서 제거: unset CASE_INSENSITIVE
fn main() {
  let args: Vec<String> = env::args().collect();
  let config = Config::new(&args).unwrap_or_else(|err| {
    println!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  println!("Searching for {}", config.query);
  println!("In file {}\n===================", config.filename);

  if let Err(e) = lib::run(config) {
    println!("Application error: {}", e);

    process::exit(1);
  }
}
