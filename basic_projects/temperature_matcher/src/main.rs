use std::io;

fn main() {
    println!("섭씨 온도를 입력하세요.");

    let mut temp = String::new();
    
    io::stdin().read_line(&mut temp)
        .expect("입력 받는데 문제가 생겼어요..ㅠ");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0
    };

    let answer = (temp * 9.0/5.0) + 32.0;

    println!("화씨 온도는 {} 입니다.", answer);

}
