use std::collections::HashMap;
use std::io;
use std::io::{BufReader, BufWriter, Bytes, Error, ErrorKind, Read, Write};
use std::net::TcpListener;

/*
    https://redis.io/docs/reference/protocol-spec/
    - RESP is essentially a serialization protocol that supports several data types.
    - In RESP, the first byte of data determines its type.
    - Clients send commands to a Redis server as an array of bulk strings.
    - The first (and sometimes also the second) bulk string in the array is the command's name.
    - Subsequent elements of the array are the arguments for the command.
    - The server replies with a RESP type.
    - Bulk string
        - The reply's type is determined by the command's implementation and possibly by the client's protocol version.
        - $5\r\nhello\r\n
        - $0\r\n\r\n
    - Array:
        - *<number-of-elements>\r\n<element-1>...<element-n>
    - Integers:
        - :1\r\n

    RESP data type	 Category	First byte
    Simple strings	 Simple 	+
    Simple Errors	 Simple 	-
    Integers	     Simple 	:
    Bulk strings	 Aggregate  $
    Arrays	         Aggregate  *
    ...

 */

fn main() {
    println!("Binding to port");
    let listener = TcpListener::bind("127.0.0.1:1337").unwrap();

    let mut db: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();

    println!("Listening for connections");
    for stream in listener.incoming() {
        println!("Client connected");
        if let Ok(stream) = stream {
            let mut writer = BufWriter::new(&stream);
            let mut reader = BufReader::new(&stream).bytes();
            loop {
                let command = match read_command(&mut reader) {
                    Ok(command) => command,
                    Err(e) => {
                        println!("Failed reading command: {e:?}");
                        break;
                    }
                };
                println!("Got command: {command:?}");
                if command.is_empty() {
                    write!(writer, "$-1\r\n").expect("Couldn't write");
                    if let Err(_) = writer.flush() {
                        println!("Couldn't write, client disconnected?");
                        break;
                    }
                    continue;
                }
                if command[0] == b"PING" {
                    write!(writer, "+PONG\r\n").expect("Couldn't write");
                } else if command[0] == b"COMMAND" && command.len() > 1 && command[1] == b"HELP" {
                    write!(writer, "*1\r\n").expect("Couldn't write");
                    write!(writer, "+HELP?\r\n").expect("Couldn't write");
                } else if command[0] == b"SET" {
                    println!("command len: {}", command.len());
                    if command.len() < 3 {
                        write!(writer, "-SET needs KEY and VALUE! :(\r\n").expect("Couldn't write");
                    } else {
                        let key = command[1].clone();
                        let value = command[2].clone();
                        println!("Storing: {value:?} under {key:?}");
                        db.insert(key, value);
                        write!(writer, "+OK\r\n").expect("Couldn't write");
                    }
                } else if command[0] == b"GET" {
                    if command.len() != 2 {
                        write!(writer, "-GET needs KEY! :(\r\n").expect("Couldn't write");
                    } else {
                        let key = command[1].clone();
                        println!("Getting: {key:?}");
                        match db.get(&key) {
                            None => write!(writer, "$-1\r\n").expect("Couldn't write"),
                            Some(value) => {
                                write!(writer, "${}\r\n", value.len()).expect("Couldn't write");
                                writer.write_all(value).expect("Couldn't write");
                                write!(writer, "\r\n").expect("Couldn't write");
                            }
                        }
                    }
                } else {
                    write!(writer, "-unknown command\r\n").expect("Couldn't write");
                }
                if let Err(_) = writer.flush() {
                    println!("Couldn't write, client disconnected?");
                    break;
                }
            }
        }
        println!("Good bye?");
    }
}

fn read_command<T>(reader: &mut Bytes<T>) -> io::Result<Vec<Vec<u8>>> where T: Read {
    match reader.next() {
        Some(Ok(b'*')) => {
            let num = read_number(reader);
            let mut command: Vec<Vec<u8>> = Vec::new();
            for _ in 0..num {
                command.push(read_string(reader));
            }
            return Ok(command);
        }
        Some(n) => { print!("Expected start of array (*), got {n:?}"); }
        None => {}
    }
    Err(Error::new(ErrorKind::Other, "Expected command"))
}

fn read_number<T>(reader: &mut Bytes<T>) -> u32 where T: Read {
    let mut num = 0;
    while let Some(Ok(b)) = reader.next() {
        let b = b as char;
        if b.is_ascii_digit() {
            num *= 10;
            num += b.to_digit(10).unwrap();
        } else {
            reader.next();
            break;
        }
    }
    println!("Got num: {num}");
    num
}

fn read_string<T>(reader: &mut Bytes<T>) -> Vec<u8> where T: Read {
    let mut string: Vec<u8> = Vec::new();
    if let Some(Ok(b'$')) = reader.next() {
        let num = read_number(reader);
        for _ in 0..num {
            string.push(reader.next().unwrap().unwrap())
        }
        reader.next();
        reader.next();
    }
    println!("Got string: {string:?}");
    string
}
