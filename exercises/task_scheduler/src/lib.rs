pub mod scheduler_message {
    use std::error;
    use std::fmt::{Debug, Display, Formatter};
    use std::io::{BufRead, BufReader, Read};

    #[derive(PartialEq)]
    pub enum Message {
        LockReqFrom(u8),
        LockReleaseFrom(u8),
        LockReq,
        LockGrant,
        LockRelease,
        Err(String),
        Done,
    }

    impl From<&str> for Message {
        fn from(msg: &str) -> Self {
            if msg.starts_with("req_lock:") {
                let sender = match msg.split(":").nth(1) {
                    Some(num) => num.parse::<u8>(),
                    None => return Message::Err(msg.to_string()),
                };
                return match sender {
                    Ok(num) => Message::LockReqFrom(num),
                    Err(e) => Message::Err(format!("{}",e)),
                };
            }
            return match msg {
                "lock_granted" => Message::LockGrant,
                "lock_release" => Message::LockRelease,
                "req_lock" => Message::LockReq,
                "done" => Message::Done,
                _ => Message::Err(msg.to_string()),
            };
        }
    }

    impl Debug for Message {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}",self)
        }
    }

    impl error::Error for Message {}

    impl Display for Message {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            return match self {
                Message::LockReqFrom(sender) => write!(f,"req_lock:{}",sender.to_string()),
                Message::LockReleaseFrom(sender) => write!(f,"lock_release:{}",sender.to_string()),
                Message::LockReq => write!(f,"req_lock"),
                Message::LockGrant => write!(f,"lock_granted"),
                Message::LockRelease =>  write!(f,"lock_release"),
                Message::Done =>  write!(f,"done"),
                Message::Err(what) =>  write!(f,"error:{}",what),
            };
        }
    }

    pub struct MessageReader<T> {
        reader: BufReader<T>,
        //buf : String,
    }

    impl<T: Read> MessageReader<T> {
        pub fn new(inner: T) -> Self {
            MessageReader { reader: BufReader::new(inner) /*buf:String::default()*/ }
        }

        pub fn read(&mut self) -> Message {
            let mut buf = String::new();
            return match self.reader.read_line(&mut buf) {
                Ok(n) => {
                    if n == 0 {
                        return Message::Err(format!("Pipe closed"));
                    }
                    trim_newline(&mut buf);
                    Message::from(buf.as_str())
                },
                Err(e) => Message::Err(format!("Error in pipe read:{}", e)),
            };
        }
    }

    fn trim_newline(s: &mut String) {
        if s.ends_with('\n') {
            s.pop();
            if s.ends_with('\r') {
                s.pop();
            }
        }
    }
}

