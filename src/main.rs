use std::{fs::File, io::{Read, Write}};
use std::fs::OpenOptions;
fn main() {
    // let mut hosts_file = fs::OpenOptions::new()
    //     .open("C://Windows//System32//drivers//etc//hosts")
    //     .write(true)
    //     .append(true)
    //     .unwrap()
    //     .expect("Something went wrong!");

    let mut hosts_file = OpenOptions::new()
        .append(true)
        .write(true)
        .read(true)
        .open(r"C:\Windows\System32\drivers\etc\hosts")
        .expect("Something went wronfg");

    let mut file_content = String::from("");
    hosts_file.read_to_string(&mut file_content)
        .expect("Something went wronasfg!");
    

    if file_content.contains("108.28.171.171 s.optifine.net") {
        println!("\n\nAlready linked!");
    }else {
        write_host(&mut hosts_file);
    }

    println!("\n\n\n\nPlease exit");
    while true {

    }
}


fn write_host(file: &mut File) {
    file.write_all(b"108.28.171.171 s.optifine.net");
    println!("Done, enjoy your cape :)");
}