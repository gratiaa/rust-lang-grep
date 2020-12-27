use std::env;
use std::fs;

// 다음 명령어로 실행해보기: cargo run the poem.txt
fn main() {
  let args: Vec<String> = env::args().collect();

  let config = parse_config(&args);

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

fn parse_config(args: &[String]) -> Config {
  /*
    args 변수의 주인(owner)은 main이며, 이를 parse_config에 빌려주고 있다.
    Config가 args의 값에 대한 오너십을 가져가려고 한다면, Rust의 규칙을 위반한 셈이 된다.

    이를 예방하는 가장 쉬운 방법은 변수에서 clone 메소드를 호출하는 것이다.
    문자열 데이터 참조값을 저장하는 시간과 메모리 비용보다 지출이 크지만
    참조의 생명주기를 관리할 필요가 없기 때문에 코드가 매우 직관적이다.
    -> 약간의 성능을 대가로 단순함을 얻는다.

    그렇지만 clone을 사용하는 것은 많은 러스트 사용자들이 지양하고 있는 방법인데, 런타임 비용이기 때문이다.
    13장에서 이보다 더 효율적인 방법에 대해 알아본다.
    여기서는 파일명과 쿼리 문자열이 아주 작고, 복사본을 한번만 사용하기 때문에 그냥 한다.
  */
  let query = args[1].clone();
  let filename = args[2].clone();

  Config { query, filename }
}
