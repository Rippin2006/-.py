use std::io::{stdin, stdout, Write};

fn main() {
    // 사용자로부터 암호화된 메시지를 입력 받음
    print!("평문 메시지 입력하시오: ");

    stdout().flush().unwrap();
    
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    // 사용자로부터 복호화할 키 값을 입력 받음
    print!("복호화할 키 값을 입력하시오: ");

    stdout().flush().unwrap();

    let mut key_input = String::new();

    stdin().read_line(&mut key_input).unwrap();

    let key = key_input.trim().parse::<u8>().unwrap();

    // 메시지를 복호화함
    let mut decrypted = String::new();

    for c in input.chars() {
        if c.is_ascii_alphabetic() {
            let mut decrypted_char = c as u8 - key;
            if decrypted_char < b'A' {
                decrypted_char += 26;
            }
            decrypted.push(decrypted_char as char);
        } else {
            decrypted.push(c);
        }
    }

    // 복호화된 메시지를 출력
    println!("복호화된 메시지: {}", decrypted); 
}

