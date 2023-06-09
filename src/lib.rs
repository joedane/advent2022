use std::io::{self, prelude::*};

pub(crate) trait Puzzle {
    fn solve<R: BufRead, W: Write>(&self, scan: &mut Scanner<R>, w: &mut W) -> io::Result<()>;
}

#[allow(dead_code)]
pub(crate) fn run<P: Puzzle>(puzzle: P) -> io::Result<()> {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(io::BufReader::new(stdin.lock()));
    let mut out = io::BufWriter::new(stdout.lock());
    puzzle.solve(&mut scan, &mut out)
}
struct Scanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitAsciiWhitespace<'static>,
}

impl<R: BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    fn token<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = std::str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
