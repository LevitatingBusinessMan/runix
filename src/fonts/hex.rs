pub const UNSCII_8: [[u8; 8]; 127] = parse::<8, 127>(include_str!("../static/fonts/unscii-8-fantasy.hex"));

/// parse a hex font
pub const fn parse<const S: usize, const N: usize>(source: &str) -> [[u8; S]; N] {
    let bytes = source.as_bytes();
    let mut i = 0;
    
    let line_length = 5 + S * 2;
    
    let mut buf: [[u8; S]; N] = [[0; S]; N];
    
    while i < N {
        let byte_index = i * line_length + 5;
        let mut j = 0;
        while j < S {
            let first = bytes[byte_index+j];
            let second = bytes[byte_index+j+1];
            let byte = (hex_value(first) << 4) | hex_value(second);
            buf[i][j] = byte;
            j += 2;
        }
        i +=1;
    }
    buf
}

/// convert a hex char to its byte value
const fn hex_value(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'a'..=b'f' => byte - b'a' + 10,
        b'A'..=b'F' => byte - b'A' + 10,
        _ => 0,
    }
}
