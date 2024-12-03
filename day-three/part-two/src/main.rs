use anyhow::Result;

enum EnableState {
    D,
    O,
    N,
    Apostrophe,
    T,
    OpenParen(bool),
    None,
}

enum MulState {
    M,
    U,
    L,
    OpenParen,
    Num1,
    Comma,
    Num2,
    None,
}

#[derive(Debug, Copy, Clone)]
struct Mul {
    num1: u32,
    num2: u32,
}

fn main() -> Result<()> {
    let data = include_str!("input.txt");

    let mut muls: Vec<Mul> = Vec::new();

    let mut mul_state = MulState::None;
    let mut enable_state = EnableState::None;
    let mut enabled = true;
    let mut num1 = String::new();
    let mut num2 = String::new();
    for c in data.chars() {
        match (&enable_state, c) {
            (_, 'd') => enable_state = EnableState::D,
            (EnableState::D, 'o') => enable_state = EnableState::O,
            (EnableState::O, '(') => enable_state = EnableState::OpenParen(true),
            (EnableState::O, 'n') => enable_state = EnableState::N,
            (EnableState::N, '\'') => enable_state = EnableState::Apostrophe,
            (EnableState::Apostrophe, 't') => enable_state = EnableState::T,
            (EnableState::T, '(') => enable_state = EnableState::OpenParen(false),
            (EnableState::OpenParen(enable), ')') => enabled = *enable,
            (_, _) => enable_state = EnableState::None,
        }
        match (&mul_state, c) {
            (_, 'm') => mul_state = MulState::M,
            (MulState::M, 'u') => mul_state = MulState::U,
            (MulState::U, 'l') => mul_state = MulState::L,
            (MulState::L, '(') => mul_state = MulState::OpenParen,
            (MulState::OpenParen, '0'..='9') => {
                mul_state = MulState::Num1;
                num1.push(c);
            }
            (MulState::Num1, '0'..='9') => num1.push(c),
            (MulState::Num1, ',') => mul_state = MulState::Comma,
            (MulState::Comma, '0'..='9') => {
                mul_state = MulState::Num2;
                num2.push(c);
            }
            (MulState::Num2, '0'..='9') => num2.push(c),
            (MulState::Num2, ')') => {
                if enabled {
                    muls.push(Mul {
                        num1: num1.parse::<u32>()?,
                        num2: num2.parse::<u32>()?,
                    });
                }
                println!("{:?}, enabled?: {}", muls.last().unwrap(), enabled);
                num1 = String::new();
                num2 = String::new();
                mul_state = MulState::None;
            }
            (_, _) => {
                num1 = String::new();
                num2 = String::new();
                mul_state = MulState::None;
            }
        };
    }

    let mut sum = 0;
    for mul in muls {
        sum += mul.num1 * mul.num2;
    }

    println!("Sum is: {}", sum);

    Ok(())
}
