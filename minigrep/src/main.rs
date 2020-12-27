use std::env;
use std::fs;

// 다음 명령어로 실행해보기: cargo run the poem.txt
fn main() {
  let args: Vec<String> = env::args().collect();

  let config = Config::new(&args);

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

/*
  에러 메시지 개선

  이제 다시 'cargo run' 해보세요!
*/
impl Config {
  fn new(args: &[String]) -> Config {
    if args.len() < 3 {
      panic!("not enough arguments")
    }
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
  }
}

/*
  TODO: 9장에서 배운 Result 반환을 코드에 적용해보기
*/
