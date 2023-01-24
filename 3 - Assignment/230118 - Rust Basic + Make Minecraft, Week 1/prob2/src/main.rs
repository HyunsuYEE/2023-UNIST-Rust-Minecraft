fn main(){
    // 일단 연산자들을  "문자열로" 받음
    // for문으로 연산자가 나오기 전까지 stack을 쌓음.
    let s = "48+75-/".to_string();

    
    
    let mut stack: Vec<u32> = Vec::new();
    for i in 0..s.len(){
        let b: u8 = s.as_bytes()[i];
        match b as char{
            '+' => {
                let first = stack.pop().unwrap();
                let second: u32 = stack.pop().unwrap();
                stack.push(first + second);
                
            },
            '-' => {
                let first = stack.pop().unwrap();
             
                let second: u32 = stack.pop().unwrap();
              
                stack.push(second - first);
                

            },
            '*' => {
                let first = stack.pop().unwrap();
                let second: u32 = stack.pop().unwrap();
                stack.push(first * second);
                
            },
            '/' => {
                let first = stack.pop().unwrap();
                let second: u32 = stack.pop().unwrap();
                stack.push(second / first);
            },
            
            _  => {stack.push((b as char)as u32 - 48);
            },

        };
        

    }
    println!("{}",stack[0]);

}
