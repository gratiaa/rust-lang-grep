use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }
    let query = args[1].clone();
    let filename = args[2].clone();

    /*
      We don’t care about the value of the environment variable,
      just whether it’s set or unset, so we’re checking is_err rather than
      using unwrap, expect, or any of the other methods we’ve seen on Result.
    */
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    Ok(Config {
      query,
      filename,
      case_sensitive,
    })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  let results = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };

  for line in results {
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  // to_lowercase(): query를 전부 소문자로 변환한다. 호출하면 기존 데이터를 참조하는 대신, 새로운 데이터를 만든다.
  let query = query.to_lowercase();
  let mut results = Vec::new();

  for line in contents.lines() {
    // contains는 string slice를 받도록 정의되어 있기 때문에 &를 앞에 붙여야 한다.
    if line.to_lowercase().contains(&query) {
      results.push(line)
    }
  }

  results
}

// 'cargo test'로 실행
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    // 'Duct tape'는 'duct'를 포함하지 않은 것으로 판단해야 한다.
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    // 'Trust me'는 'rUsT'를 포함한 것으로 판단해야 한다.
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    );
  }
}

/*
 TODO:
 - standard output 대신에 standard error로 에러 메시지 출력하기
  - println! 매크로는 standard output만 사용 가능하다.
  - 다음 명령어를 실행해보자: cargo run > output.txt
    - >를 사용하면 standard output으로 하여금 컨텐츠를 output.txt에 넣도록 한다.
    - 에러 메시지는 standard output을 통해 파일 생성 말고, 따로 출력되게 하고 싶다!
*/
