use std::ffi::c_int;
use std::fs::read;
use std::io;

fn main() {
    println!("Hello, world!");

    // inputs
    let mut input = String::new();
    let mut result: i32 = 0;

    input = ReadLine().trim().to_string();

    let mut seperate = Seperate(input);
    //println!("{:?}", seperate);

    let processed = ProcessBrackets(seperate);
    //println!("{:?}", processed);
    let Calculator = match CalculateFormula(processed) {
        Ok(Calculator) => Calculator,
        Err(error) => panic!("{}", error),
    };

    println!("계산 결과: {:.5}", Calculator);
}

fn VaildateInput(input: &Vec<String>) -> Result<(), String> {
    let mut depth = 0;

    for i in input {
        match i.as_str() {
            "(" => depth += 1,
            ")" => {
                depth -= 1;
                if depth < 0 {
                    return Err("괄호가 올바르지 않습니다.".to_string());
                }
            }
            _ => {}
        }
    }

    if depth != 0 {
        return Err("괄호가 올바르지 않습니다!".to_string());
    }

    Ok(())
}

fn ReadLine() -> String{
    let mut result = String::new();

    io::stdin().read_line(&mut result).unwrap();
    result = result.trim().to_string();

    if result.is_empty() {
        println!("입력값이 없습니다. 재 입력을 시도합니다.");
        return ReadLine();
    }

    result
}


// String으로 받아서 토큰으로 분리하는것, 후위 표기식 X
fn Seperate(str: String) -> Vec<String> {
    let mut result = Vec::new();
    let mut temp = String::new();

    for c in str.chars() {
        if c.is_numeric() || c == '.' {
            temp.push(c);
        } else if c == '-' {
            if result.is_empty()
                || result.last() == Some(&"+".to_string())
                || result.last() == Some(&"-".to_string())
                || result.last() == Some(&"*".to_string())
                || result.last() == Some(&"/".to_string())
                || result.last() == Some(&"(".to_string())
            {
                temp.push(c);
            } else {
                if !temp.is_empty() {
                    result.push(temp.clone());
                    temp.clear();
                }
                result.push(c.to_string());
            }
        } else if c == ' ' {
            continue;
        } else {
            if c == '+' || c == '*' || c == '/' || c == '(' || c == ')' {
                if (!temp.is_empty()) {
                    result.push(temp.clone());
                    temp.clear();
                }
                result.push(c.to_string());
            }

            temp.clear();
        }
    }
    if !temp.is_empty() {
        result.push(temp.clone());
    }

    result
}


// 식 계산기, 이거 자체로만은 괄호 연산이 불가.
// 괄호 연산하려면 ProcessBrackets로 사용
fn CalculateFormula(str: Vec<String>) -> Result<f32, String> {
    let mut result: f32 = 0.0;

    let mut preNum = 0.0;

    let mut multipleCalResult: Vec<String> = Vec::new();

    let mut i = 0;
    while i < str.len() {
        match str[i].as_str() {
            "*" => {
                let leftNum = multipleCalResult.pop().unwrap();
                let rightNum = str[i + 1].as_str().parse::<f32>().unwrap();



                multipleCalResult.push((leftNum.parse::<f32>().unwrap() * rightNum).to_string());
                i += 2;
            }
            "/" => {
                let leftNum = multipleCalResult.pop().unwrap();
                let rightNum = str[i + 1].as_str().parse::<f32>().unwrap();

                if (rightNum == 0.0) {
                    return Err("0으로 나눌 수 없습니다.".to_string())
                }
                multipleCalResult.push((leftNum.parse::<f32>().unwrap() / rightNum).to_string());
                i += 2;
            }
            _ => {
                multipleCalResult.push(str[i].clone());
                i += 1;
            }
        }
    }

    i = 0;
    while i < multipleCalResult.len() {
        match multipleCalResult[i].as_str() {
            "+" => {
                preNum = preNum + multipleCalResult[i + 1].as_str().parse::<f32>().unwrap();
                i += 2;
            }
            "-" => {
                preNum = preNum - multipleCalResult[i + 1].as_str().parse::<f32>().unwrap();
                i += 2;
            }
            _ => {
                preNum = multipleCalResult[i].as_str().parse::<f32>().unwrap();
                i += 1;
            }
        }
    }

    result = preNum;

    return Ok(result);
}


// 괄호 처리기
// ")" 문자를 찾으면 그 문자를 기준으로 -1 하고
// "(" 문자가 나올때까지 찾아 +1 한 내부의 배열들을 Vec<String>으로 변환해
// CalculateFormula에 넘기기
fn ProcessBrackets(brackets: Vec<String>) -> Vec<String> {
    let mut _result: Vec<String> = Vec::new();

    let close = brackets.iter().position(|x| x == ")");

    if let Some(close) = close {
        let open = brackets[..close].iter().rposition(|x| x == "(");

        if let Some(open) = open {
            let inner = brackets[open + 1..close].to_vec();
            let in_result = match CalculateFormula(inner) {
                Ok(result) => result,
                Err(_) => return brackets,
            };

            let mut new = brackets[..open].to_vec();
            new.push(in_result.to_string());
            new.extend(brackets[close + 1..].to_vec());

            return ProcessBrackets(new);
        }
    } else {
        return brackets;
    }

    brackets
}
