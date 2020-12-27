use std::env;
use std::fs;
use std::process;

// 다음 명령어로 실행해보기: cargo run the poem.txt
fn main() {
  let args: Vec<String> = env::args().collect();
  let config = Config::new(&args).unwrap_or_else(|err| {
    println!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  println!("Searching for {}", config.query);
  println!("In file {}", config.filename);

  run(config);
}

/*
  main 함수에서 실행 로직을 분리한다.
*/
fn run(config: Config) {
  let contents =
    fs::read_to_string(config.filename).expect("Something went wrong while reading the file");

  println!("With text:\n{}", contents);
}

struct Config {
  query: String,
  filename: String,
}

impl Config {
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
  TODO: run 에러 처리 보충
*/
