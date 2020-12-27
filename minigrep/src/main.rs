use std::env;
use std::fs;
// process 표준 라이브러리 불러온다.
use std::process;

// 다음 명령어로 실행해보기: cargo run
fn main() {
  let args: Vec<String> = env::args().collect();

  /*
    unwrap_or_else
    : non-panic! 에러 핸들링할 때 사용한다.
    Result가 Ok -> unwrap과 동일한 동작을 한다.
    Result가 Err -> closure(13장에서 다룬다) 안의 코드를 호출한다.
  */
  let config = Config::new(&args).unwrap_or_else(|err| {
    println!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  println!("Searching for {}", config.query);
  println!("In file {}", config.filename);

  let contents =
    fs::read_to_string(config.filename).expect("Something went wrong while reading the file");

  println!("With text:\n{}", contents);
}

struct Config {
  query: String,
  filename: String,
}

// panic! 호출 대신에 Result 반환
impl Config {
  // &'static str: 스트링 리터럴 타입
  // -> https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#the-static-lifetime 참고
  fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }
    let query = args[1].clone();
    let filename = args[2].clone();

    Ok(Config { query, filename })
  }
}

/*
  TODO: run 함수 추출하기.
*/
