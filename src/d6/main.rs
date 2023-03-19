use anyhow::Result;

fn found_marker(buf: &[u8]) -> bool {
    for i in 0..buf.len() - 1 {
        for j in i + 1..buf.len() {
            if buf[i] == buf[j] {
                return false;
            }
        }
    }
    return true;
}

fn _find_packet_marker(data: &[u8]) -> Result<usize> {
    find_marker(data, 4)
}

fn find_message_marker(data: &[u8]) -> Result<usize> {
    find_marker(data, 14)
}

fn find_marker(data: &[u8], size: usize) -> Result<usize> {
    let mut buf: Vec<u8> = data[0..size].to_owned();
    let mut pos = size;
    loop {
        if found_marker(&buf) {
            return Ok(pos);
        }
        buf[pos % size] = data[pos];
        pos += 1;
    }
}
fn main() -> Result<()> {
    let data = include_bytes!("input.txt");
    //let data = b"mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    //let data = b"bvwbjplbgvbhsrlpgdmjqwftvncz";
    //let data = b"nppdvjthqldpwncqszvftbrmjlhg";
    //let data = b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    //let data = b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    match find_message_marker(data) {
        Ok(pos) => {
            let marker = &data[(pos - 4)..pos];
            println!("found marker {} at pos {pos}", std::str::from_utf8(marker)?);
            Ok(())
        }
        Err(e) => Err(e),
    }
}

mod test {}
