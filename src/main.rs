fn main() {

    let s = "hello";


    let result = check_regex("abc", "abc");
    println!("{}", result);

}

fn check_regex(regex: &str, input: &str) -> bool {

    let mut i = 0;
    let mut current_state: char = regex.as_bytes()[i] as char; // a

    for character in input.chars() {

        if current_state == character && i < regex.len() { // c
            if i == regex.len() - 1 {
                return true;
            }
            i += 1; // 3
            current_state = regex.as_bytes()[i] as char
        } else {
            i = 0;
            current_state = regex.as_bytes()[i] as char
        }

    }

    return false;
}