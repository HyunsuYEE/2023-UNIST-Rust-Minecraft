fn main() {
    let two_dim_arr:[String;4] = [" * * ".to_string(),
                                  "  *  ".to_string(),
                                  "  *  ".to_string(),
                                  "     ".to_string()];

    
    
    let mut two_dim_vec : Vec<String> = Vec::new();
    for i in 0..two_dim_arr.len(){
        let mut vec: Vec<String> = Vec::new();
        for j in 0..5 as usize{
            match i {
                0 =>{
                    let mut count : i32 = 0;
                    match j {
                        0 => {if two_dim_arr[i].as_bytes()[j] as char  == '*' {vec.push(String::from('*'));
                                                                                continue;}
                              if two_dim_arr[i].as_bytes()[j+1] as char == '*' {count = count + 1;}
                              if two_dim_arr[i+1].as_bytes()[j] as char == '*' {count = count + 1;}
                              if two_dim_arr[i+1].as_bytes()[j+1] as char == '*' {count = count + 1;}
                              

                              if count == 0 {vec.push(String::from(' '));
                                               continue;}
                              vec.push(count.to_string());
                              continue;
                    
                    
                    
                    },
                        4 => {if two_dim_arr[i].as_bytes()[j] as char  == '*' {vec.push('*'.to_string());
                                                                                continue;}
                              if two_dim_arr[i].as_bytes()[j-1] as char == '*' {count = count + 1;}
                              if two_dim_arr[i+1].as_bytes()[j] as char == '*' {count = count + 1;}
                              if two_dim_arr[i+1].as_bytes()[j-1] as char == '*' {count = count + 1;}

                              if count == 0 {vec.push(String::from(' '));
                                               continue;}
                              vec.push(count.to_string());
                              continue;},
                        _ => {if two_dim_arr[i].as_bytes()[j] as char  == '*' {vec.push('*'.to_string());
                                                                                continue;}
                              if two_dim_arr[i].as_bytes()[j-1] as char == '*' {count = count + 1;}
                              if two_dim_arr[i+1].as_bytes()[j-1] as char == '*' {count = count + 1;}
                              if two_dim_arr[i+1].as_bytes()[j] as char == '*' {count = count + 1;}
                              if two_dim_arr[i+1].as_bytes()[j+1] as char == '*' {count = count + 1;}
                              if two_dim_arr[i].as_bytes()[j+1] as char == '*' {count = count + 1;}

                              if count == 0 {vec.push(String::from(' '));
                                               continue;}
                              vec.push(count.to_string());
                              continue;}
                    }
                },////////////////////////////////////////////////////////////////////////////////////////////
                3 =>{
                    let mut count : i32 = 0;
                    match j {
                        0 => {if two_dim_arr[i].as_bytes()[j] as char  == '*' {vec.push('*'.to_string());
                                                                                continue;}
                              if two_dim_arr[i-1].as_bytes()[j+1] as char == '*' {count = count + 1;}
                              if two_dim_arr[i-1].as_bytes()[j] as char == '*' {count = count + 1;}
                              if two_dim_arr[i].as_bytes()[j+1] as char == '*' {count = count + 1;}

                              if count == 0 {vec.push(String::from(' '));
                                               continue;}
                              vec.push(count.to_string());
                              continue;
                    
                    
                    
                    },
                        4 => {if two_dim_arr[i].as_bytes()[j] as char  == '*' {vec.push('*'.to_string());
                                                                                continue;}
                              if two_dim_arr[i].as_bytes()[j-1] as char == '*' {count = count + 1;}
                              
                              if two_dim_arr[i-1].as_bytes()[j] as char == '*' {count = count + 1;}
                              if two_dim_arr[i-1].as_bytes()[j-1] as char == '*' {count = count + 1;}

                              if count == 0 {vec.push(String::from(' '));
                                               continue;}
                              vec.push(count.to_string());
                              continue;},
                        _ => {if two_dim_arr[i].as_bytes()[j] as char  == '*' {vec.push('*'.to_string());
                                                                                continue;}
                              if two_dim_arr[i].as_bytes()[j-1] as char == '*' {count = count + 1;}
                              if two_dim_arr[i-1].as_bytes()[j-1] as char == '*' {count = count + 1;}
                              if two_dim_arr[i-1].as_bytes()[j] as char == '*' {count = count + 1;}
                              if two_dim_arr[i-1].as_bytes()[j+1] as char == '*' {count = count + 1;}
                              if two_dim_arr[i].as_bytes()[j+1] as char == '*' {count = count + 1;}

                              if count == 0 {vec.push(String::from(' '));
                                               continue;}
                              vec.push(count.to_string());
                              continue;}
                    }
                },////////////////////////////////////////////////////////////////////////////////////////////
                _ =>{
                    let mut count : i32 = 0;
                    match j {
                        0 => {if two_dim_arr[i].as_bytes()[j] as char  == '*' {vec.push('*'.to_string());
                                                                                continue;}
                              if two_dim_arr[i-1].as_bytes()[j] as char == '*' {count = count + 1;}
                              if two_dim_arr[i-1].as_bytes()[j+1] as char == '*' {count = count + 1;}
                              if two_dim_arr[i].as_bytes()[j+1] as char == '*' {count = count + 1;}
                              if two_dim_arr[i+1].as_bytes()[j+1] as char == '*' {count = count + 1;}
                              if two_dim_arr[i+1].as_bytes()[j] as char == '*' {count = count + 1;}

                              if count == 0 {vec.push(String::from(' '));
                                               continue;}
                              vec.push(count.to_string());
                              continue;
                    
                    
                    
                    },
                        4 => {if two_dim_arr[i].as_bytes()[j] as char  == '*' {vec.push('*'.to_string());
                                                                                continue;}
                              if two_dim_arr[i-1].as_bytes()[j] as char == '*' {count = count + 1;}
                              if two_dim_arr[i-1].as_bytes()[j-1] as char == '*' {count = count + 1;}
                              if two_dim_arr[i].as_bytes()[j-1] as char == '*' {count = count + 1;}
                              if two_dim_arr[i+1].as_bytes()[j-1] as char == '*' {count = count + 1;}
                              if two_dim_arr[i+1].as_bytes()[j] as char == '*' {count = count + 1;}

                              if count == 0 {vec.push(String::from(' '));
                                               continue;}
                              vec.push(count.to_string());
                              continue;},
                        _ => {if two_dim_arr[i].as_bytes()[j] as char  == '*' {vec.push('*'.to_string());
                                                                                continue;}
                              if two_dim_arr[i-1].as_bytes()[j] as char == '*' {count = count + 1;}
                              if two_dim_arr[i-1].as_bytes()[j-1] as char == '*' {count = count + 1;}
                              if two_dim_arr[i].as_bytes()[j-1] as char == '*' {count = count + 1;}
                              if two_dim_arr[i+1].as_bytes()[j-1] as char == '*' {count = count + 1;}
                              if two_dim_arr[i+1].as_bytes()[j] as char == '*' {count = count + 1;}
                              if two_dim_arr[i+1].as_bytes()[j+1] as char == '*' {count = count + 1;}
                              if two_dim_arr[i].as_bytes()[j+1] as char == '*' {count = count + 1;}
                              if two_dim_arr[i-1].as_bytes()[j+1] as char == '*' {count = count + 1;}

                              if count == 0 {vec.push(String::from(' '));
                                               continue;}
                              vec.push(count.to_string());
                              continue;}
                    }
                }

            }
        }
        two_dim_vec.push(vec.iter().map(ToString::to_string).collect());
    }
    for i in two_dim_vec.iter() {
    println!("{}", i);
    }
    

}
