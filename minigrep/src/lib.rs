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

  for line in search(&config.query, &contents) {
    println!("{}", line);
  }

  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  // lines()는 iterator를 반환한다. (iterator에 대해서는 13장에서 자세히 설명)
  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }

  results
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
 TODO: (커맨드라인 프로그램 작성할 때 도움이 되는 지식들)
 1. 환경 변수 다루는 법: 환경 변수로 플래그 받아서 search_case_insensitive 함수
 2. standard error 출력하는 법
*/
