use std::error::Error;
use std::fs;

pub struct Config {
  pub query: String,
  pub filename: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }
    let query = args[1].clone();
    let filename = args[2].clone();

    Ok(Config { query, filename })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  println!("With text:\n{}", contents);

  Ok(())
}

/*
  lifetime 파라미터를 사용하여 인자의 lifetime을 리턴 값 lifetime과 연결한다.
  -> 중요: Rust로 하여금 search 함수가 반환한 데이터는 contents 인자로 넘어간 데이터의 생애와 같다고 알려주는 것.

  Quiz: lifetime 어노테이션 없이 컴파일하면 어떻게 될까?
*/
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  vec![]
}

// 'cargo test'로 실행
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }
}

/*
 TODO: 테스트 코드 통과
*/
