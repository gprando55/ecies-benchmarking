use std::{path::Path, io, fs::File};
use ecies::{decrypt, encrypt, utils::generate_keypair};


fn main() {
    const MSG: &str = "helloworld";
    let (sk, pk) = generate_keypair();
    let (sk, pk) = (&sk.serialize(), &pk.serialize());

    let msg = MSG.as_bytes();

    assert_eq!(
        msg,
        decrypt(sk, &encrypt(pk, msg).unwrap()).unwrap().as_slice()
    );
    println!("Hello, world!");


    if let Ok(lines) = read_lines("./data.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(data) = line {
                let values: Vec<&str> = data.split(' ').collect();
                match values.len() {
                  2 => {
                    let strdata = values[0].parse::<String>();
                    let intdata =  values[1].parse::<i32>();
                    println!("Got: {:?} {:?}", strdata, intdata);
                  },
                  _ => panic!("Invalid input line {}", data),
                };
            }
        }
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<std::io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).buffer())
}