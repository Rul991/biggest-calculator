use std::{fs::File, io::Write};

fn create_code(min: i32, max: i32) -> String
{
    let mut code = String::from("a = int(input('a: '))\nb = int(input('b: '))\n\n");
    let mut is_use_elif = false;

    for i in min..=max {
        for j in min..=max {
            let condition_key_word = 
                if is_use_elif { "elif "}
                else { "if" };

            code.push_str(&format!("{condition_key_word} a == {i} and b == {j}: print('{i} + {j} = {}')\n", i+j));
            is_use_elif = true;
        }
    }

    code.push_str("else: print('not supported value')");

    code
}

fn create_python_file(src: &str, min: i32, max: i32) -> bool
{
    let mut file = File::open(src)
        .unwrap_or(
            File::create(src)
                .unwrap()
        );
    let code = create_code(min, max);

    file.write_all(code.as_bytes());

    true
}

fn main() 
{
    let _ = create_python_file("main.py", 0, 100);
}