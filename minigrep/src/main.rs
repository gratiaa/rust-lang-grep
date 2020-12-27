use std::env;
// 파일을 다루는 표준 라이브러리를 추가한다.
use std::fs;

// 다음 명령어로 실행해보기: cargo run the poem.txt
fn main() {
  let args: Vec<String> = env::args().collect();

  let query = &args[1]; // query는 다음에 사용한다.
  let filename = &args[2];

  println!("In file {}", filename);

  let contents = fs::read_to_string(filename).expect("Something went wrong while reading the file");

  println!("With text:\n{}", contents)
}

/*
  https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#refactoring-to-improve-modularity-and-error-handling
  위 main 함수의 문제점은?

  1. 많은 일을 한 함수 안에서 한다. 프로그램이 커질 수록 유지보수에 좋지 않다.
  -> 기능별로 함수 분리가 필요하다.

  2. 프로그램 안의 변수와 설정 변수(query, filename)가 섞일 염려가 있다.
  -> 설정 변수를 하나의 구조로 묶어야 한다.

  3. expect를 쓰긴 썼으나, 출력 정보가 충분하지 않다.
  -> 좀 더 에러 해결에 도움이 되는 정보를 주도록 출력한다.

  4. 프로그램 여기저기 expect를 해두고 충분한 인자를 주지 않은채 프로그램을 실행 하면,
  에러 내용이 원인을 설명하기에 충분하지 못한 내용으로 출력된다. (index out of bounds Error)
  -> 에러 다루는 로직을 한 곳에 몰아두어 유지보수하기 쉽게 만들어야 한다.

  TODO: 리팩토링
*/
