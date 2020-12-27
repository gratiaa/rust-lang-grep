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
