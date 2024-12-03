use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};

struct Parser {
    reader: BufReader<File>,
    prev: u8,
    enabled: bool,
    sum: i32,
}

impl Parser {
    fn new(file: File) -> Self {
        let reader = BufReader::new(file);
        Parser {
            reader,
            prev: b'0',
            enabled: true,
            sum: 0,
        }
    }

    fn read_next(&mut self) -> bool {
        let mut buf = [0];
        let bytes_read = match self.reader.read(&mut buf) {
            Ok(byte) => byte,
            Err(_) => 0,
        };

        if bytes_read > 0 {
            self.prev = buf[0];
            return true;
        } else {
            return false;
        }
    }

    fn go_back(&mut self) -> bool {
        match self.reader.seek(SeekFrom::Current(-1)) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn parse_mult(&mut self) {
        let mut str = String::new();

        let mut i = 0;
        while i < 3 && self.read_next() {
            str.push(self.prev as char);
            i += 1;
        }

        if str != "ul(" {
            self.go_back();
            self.go_back();
            self.go_back();
            return;
        }

        str.clear();
        while self.read_next() && self.prev != b',' {
            if b'9' < self.prev || self.prev < b'0' {
                self.go_back();
                return;
            }
            str.push(self.prev as char);
        }

        let num: i32 = match str.parse() {
            Ok(num) => num,
            Err(_) => -1,
        };

        if num == -1 {
            return;
        }

        str.clear();
        while self.read_next() && self.prev != b')' {
            if b'9' < self.prev || self.prev < b'0' {
                self.go_back();
                return;
            }
            str.push(self.prev as char);
        }

        let num1: i32 = match str.parse() {
            Ok(num) => num,
            Err(_) => -1,
        };

        if num1 == -1 {
            return;
        }

        self.sum += num * num1;
    }

    fn parse_do(&mut self) {
        let mut str = String::new();

        while self.read_next() && str.len() < 6 {
            str.push(self.prev as char);

            if str == "o()" {
                self.enabled = true;
                break;
            }
        }

        if str == "on't()" {
            self.enabled = false;
        }

        self.go_back();
    }

    fn parse(&mut self) -> io::Result<()> {
        while self.read_next() {
            if self.prev == b'd' {
                self.parse_do();
            }
            if self.enabled && self.prev == b'm' {
                self.parse_mult();
            }
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt").expect("could not read file");
    let mut parser = Parser::new(file);

    parser.parse()?;

    println!("{}", parser.sum);

    Ok(())
}
