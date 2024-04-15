

use std::fs;
use std::io;
use std::env;
// use once_cell::sync::Lazy;  


/* 循环更新脚本
获取文件系统，设置文件路经
*/
fn get_os_type() {
    let os_type = env::consts::OS;
    println!("Operating system: {}", os_type);
    match os_type {
        "linux" => println!("This is a Linux system."),
        "macos" => println!("This is a macOS system."),
        "windows" => println!("This is a Windows system."),
        _ => println!("Unknown operating system."),
    }
}

// static OS_TYPE: Lazy<String> = Lazy::new(|| {
//     env::consts::OS
//     });
    
fn read_file_as_strings(filename: &str) -> Result<Vec<String>, io::Error> {
        let contents = fs::read_to_string(filename)?;
        let lines = contents.lines().map(|line| line.to_string()).collect::<Vec<String>>();
        Ok(lines)
}

fn main() {
    get_os_type(); // 正确调用 get_os_type 函数
    // println!("Operating system: {}", *OS_TYPE);

    let filename = "/home/pi/esl.txt";
    let result = read_file_as_strings(filename);

    match result {
        Ok(lines) => {
        for line in lines {
            println!("{}", line);
    }
}
    Err(error) => {
        println!("Error reading file: {}", error);
}
}
}