use std::io;


fn main() {
    println!("몇번째 피보나치 수를 원하는지 입력하세요.");

    let mut num = String::new();

    io::stdin().read_line(&mut num)
        .expect("입력한 값을 읽지 못했습니다.");

    let num: u32 = num.trim().parse()
        .expect("입력한 값을 변환하지 못하였습니다.");

    let mut prev = 0;
    let mut retval = 1;
    for _ in 1..num {
        let tmp = retval;
        retval = prev + retval;
        prev = tmp;
    }

    println!("결과: {}", retval);
}
