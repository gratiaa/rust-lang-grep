use std::env;
use std::fs;

/*
  https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#separation-of-concerns-for-binary-projects

  [바이너리 프로젝트 관심사 분리하기]

  * main.rs와 lib.rs로 파일을 나눈 후에 lib.rs에 프로그램 로직을 옮긴다.
    * command line parsing 로직 크기가 작다면 따로 파일을 나누지 않고, main.rs에 두어도 된다.
    * 로직 크기가 커진다면 추출하여 lib.rs로 옮긴다.

  위 과정을 거치면 main 함수에 남는 것은 다음과 같게 된다.

  * 인자 값과 함께 command line parsing 로직 호출
  * 기타 다른 설정값 세팅
  * lib.rs의 run 실행
  * run 함수가 에러를 반환하면 에러 처리
*/

// 다음 명령어로 실행해보기: cargo run the poem.txt
fn main() {
  let args: Vec<String> = env::args().collect();

  let (query, filename) = parse_config(&args);

  println!("In file {}", filename);

  let contents = fs::read_to_string(filename).expect("Something went wrong while reading the file");

  println!("With text:\n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
  let query = &args[1];
  let filename = &args[2];

  (query, filename)
}
