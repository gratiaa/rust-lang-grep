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
  Config 생성자 만들기

  이전 parse_config의 역할은 Config 인스턴스를 생성하는 것이다.
  Config struct와의 관계성 설정을 위해 따라서 함수 이름을 new로 바꾼다.
  그러면 String::new와 같이 관용적인 코드가 만들어진다.
*/
impl Config {
  fn new(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
  }
}
