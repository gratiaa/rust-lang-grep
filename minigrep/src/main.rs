use std::env;
use std::process;

// 아래와 같이 하면 에러 발생하여서
// use minigrep::Config;

// https://stackoverflow.com/questions/53677463/rust-book-12-3-unresolved-import-error-e4032
// 참고하여 아래와 같이 import
mod lib;
use lib::Config;

// 다음 명령어로 실행해보기: cargo run the poem.txt
fn main() {
  let args: Vec<String> = env::args().collect();
  let config = Config::new(&args).unwrap_or_else(|err| {
    println!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  println!("Searching for {}", config.query);
  println!("In file {}", config.filename);

  if let Err(e) = lib::run(config) {
    println!("Application error: {}", e);

    process::exit(1);
  }
}

/*
  TODO: TDD 개발로 라이브러리에 기능 추가해보기

  다음 TDD를 따릅니다.

  1. 실패하는 테스트를 작성한다. 실패 이유가 의도대로 나오는지 확인한다.
  2. 이 테스트를 통과할 정도로만 코드를 수정하거나 작성한다.
  3. 리팩토링 하면서 테스트도 계속 통과하는지 확인한다.
  4. 1번부터 다시 반복한다.

  * 추가할 기능: 문자열 쿼리를 인자로 받아서 이와 매치하는 줄을 리스트로 뽑는 search 함수
*/
