// args 함수 사용을 위해 use 문으로 std::env 모듈을 불러온다.
use std::env;

fn main() {
    /*
    1. std::env::args의 모듈 뎁스가 2인 것에 주목!
        7장에서 배웠듯이, 사용하려는 모듈이 중첩되어 있을 경우,
        그 모듈 자체말고 부모 모듈을 스코프로 불러오는 것이 관례이다.
        이렇게 하면 std::env 안의 다른 함수도 사용할 수 있기 때문이다.

    2. std::env::args에 무효한(invalid) Unicode를 인자로 넣으면 panic.
        무효한 Unicode를 넣으려면 std::env::args_os를 사용해 OsString으로 받으면 된다.
        하지만 OsString는 일반 String 값보다 다루기가 훨씬 까다로워서 이 장에서는 생략한다.
    */

    /*
    collect를 호출하여 iterator에서 생성한 값이 모두 담긴 벡터를 받는다.
    collect는 다양한 콜렉션을 만들어 내므로, args의 타입을 명확하게 기술한다.
    Rust에서 annotate는 거의 할 일이 없겠지만,
    collect 사용할 때는 Rust가 콜렉션 타입을 추론하지 못하므로 annotate 해야 한다.
    */
    let args: Vec<String> = env::args().collect();

    /*
    debug formatter :?를 사용하여 벡터를 출력한다.
    1. 먼저 'cargo run'을 실행해 본다.
    2. 그 다음, 'cargo run needle haystack'을 실행해 본다.
    */
    println!("{:?}", args);
}
