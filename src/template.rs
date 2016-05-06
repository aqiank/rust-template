// Std
use std::borrow::Borrow;
use std::fs::File;
use std::io;
use std::io::Read;
use std::result;

// Modules
use segment::Segment;
use functions::funcs;

pub type Result<T> = result::Result<T, Error>;

pub enum Error {
    IOError,
    ParseError,
}

pub struct Template<'a> {
    name: &'a str,
    buffer: String,
    pub segments: Vec<Segment>,
}

impl<'a> Template<'a> {
    pub fn render(&'a self) -> String {
        let mut output = String::new();

        // Generate output
        for segment in &self.segments {
            match segment {
                &Segment::Text(begin, end) => {
                    output.push_str(&self.buffer[begin..end]);
                },
                &Segment::Block(begin, end) => {
                    if let Some(s) = eval(&self.buffer[begin..end]) {
                        output.push_str(&s);
                    }
                },
            }
        }

        output
    }
}

enum State {
    FindingBeginBlock,
    FindingEndBlock,
}

pub fn parse<'a>(filename: &'a str) -> Result<Template> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return Err(Error::IOError),
    };

    let mut buffer = String::new();
    if let Err(_) = file.read_to_string(&mut buffer) {
        return Err(Error::IOError);
    }

    let mut segments = Vec::new();
    let mut begin = 0;
    let mut end = 0;
    let mut state = State::FindingBeginBlock;

    loop {
        // Make sure we never have an out-of-range index
        if begin >= buffer.len() {
            break;
        }

        match state {
            State::FindingBeginBlock => {
                if let Some(n) = buffer[begin..].find("{{") {
                    if begin == n {
                        state = State::FindingEndBlock;
                        continue;
                    }
                    end = begin + n; 
                    segments.push(Segment::Text(begin, end));
                    begin = end + 2;
                    state = State::FindingEndBlock;
                } else {
                    segments.push(Segment::Text(begin, buffer.len()));
                    break;
                }
            },
            State::FindingEndBlock => {
                if let Some(n) = buffer[begin..].find("}}") {
                    end = begin + n; 
                    segments.push(Segment::Block(begin, end));
                    begin = end + 2;
                    state = State::FindingBeginBlock;
                } else {
                    segments.push(Segment::Text(begin, buffer.len()));
                    break;
                }
            },
        };
    }

    Ok(Template{
        name: filename,
        buffer: buffer,
        segments: segments,
    })
}

fn eval(s: &str) -> Option<String> {
    let ref mut ss = s.split_whitespace();

    let f = if let Some(arg) = ss.next() {
        if let Some(f) = funcs.get(arg) {
            let args = ss.collect::<Vec<&str>>();
            return f(&args);
        }
    };

    None
}
