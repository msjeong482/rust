use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub struct Guess {
    value: u32
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("guessing value is between 1 ~ 100, current guessing value:{}", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

fn main() {
    println!("숫자를 맞혀봅시다");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("정답이라고 생각하는 숫자를 입력하세요.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("입력한 값을 읽지 못했습니다.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                let guess = Guess::new(num);
                guess.value()
            }
            Err(_) => continue,
        };

        println!("입력한 값: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("입력한 숫자가 작습니다!"),
            Ordering::Greater => println!("입력한 숫자가 큽니다!"),
            Ordering::Equal => {
                println!("정답!");
                break;
            }
        }
    }
}
