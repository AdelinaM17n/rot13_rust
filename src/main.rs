use std::io;

fn main() {
    println!("Please enter the text you want to encrypt or decrypt with ROT13 
if you enter text thats already encrypted with ROT13 it will get decrypted");
    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("Failed to read line");
    let char_vec: Vec<char> = string.chars().collect();
    let the_string:String = encrypt(char_vec);
    println!("{}",the_string);
}

fn encrypt(cvec: Vec<char>)-> String{
    let mut v: Vec<char> = Vec::new();
    for &i in &cvec{
        if i.is_ascii_alphabetic() && !i.is_ascii_whitespace(){
            let i_as_ascii = i as u8;
            let ascii_comparison: u8 = if i.is_ascii_lowercase() {109} else {77};
            if i_as_ascii <= ascii_comparison{
                let ascii_to_char: char = (i_as_ascii + 13) as char;
                 v.push(ascii_to_char);
            }else { 
                let ascii_to_char: char = (i_as_ascii - 13) as char;
                v.push(ascii_to_char);
            }
        }else {
            v.push(i);
        }
    }
    v.into_iter().collect()
}