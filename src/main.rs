
// 외부 라이브러리 코드 가져오기

extern crate rand;

use std::cmp::Ordering;
// io 라이브러리를 스코프로 가져옴 -> 입출력에 관한
// std라고 불리는 표준 라이브러리에 존재
use std::io;
use rand:: Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1 .. 101);

    println!("The secret number is: {} ", secret_number);

    loop {
        println!("Please input your guess.");

        // 기본적으로 러스트는 변수 생성이 불변임
        // mut (=mutable)을 붙여야 가변변수가 됨
        // ::new에서 ::는 String 타입의 연관함수
        // 연관함수 = 하나의 타입을 위한 함수 (인스턴스가 아니라)
        // 정적 메서드라고도 불림
        let mut guess = String::new();

        // &은 참조자 참조자도 기본적으로 불변임
        // 따라서 가변으로 바꾸기 위해 mut을 붙임
        // .expect은 잠재된 실패를 다룰 수 있음
        // read_lin()은 값을 리턴하는데, 이 때 돌려준 값은 io::Result임.
        // Result의 variants는 Ok와 Err인데 ok는 처리 성공, Err 처리 실패 그리고 그 이유에 대한 정보
        // Result는 에러 처리를 위한 정보를 표현하기 위해 사용
        // io::Result 인스턴스는 expect 메소드가 있음
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // 이전에 있던 값을 가리는 것 허용
        // trim을 사용하는 이유는 5를 입력하고 엔터를 칠 경우
        // 5\n이기 때문
        // trim은 처음과 끝 부분의 빈칸을 제거하는 함수
        // parse()는 문자열을 숫자형으로 파싱함.
        // 따라서 :u32처럼 정확한 타입을 적어야함
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number");


        println!("Your guessed {}", guess);


        // cmp 메소드는 두 값을 비교
        // 비교하고 싶은 것의 참조자
        // cmp는 Ordering의 열거형을 리턴ㅣ
        match guess.cmp(&secret_number) {
            Ordering:: Less => println!("Too small"),
            Ordering:: Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
    }
}
