use std::env;

// 다음 명령어로 실행해보기: cargo run test sample.txt
fn main() {
  let args: Vec<String> = env::args().collect();

  // 다른 곳에서도 사용 가능할 수 있게 두 인자의 값을 저장한다.
  let query = &args[1]; // &args[0]은 건너뛴다. -> 왜?
  let filename = &args[2];

  println!("Searching for {}", query);
  println!("In file {}", filename);
}
