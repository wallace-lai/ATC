struct Solution;

impl Solution {
    fn i32_to_hex_comp(n: i32, buf: &mut [u8; 8]) -> &[u8] {
        const HEX_LUT: &[u8; 16] = b"0123456789abcdef";
        let mut value = n as u32;
        if value == 0 {
            buf[0] = b'0';
            return &buf[..1];
        }

        // i32最多8个十六进制位
        let mut idx = 8;
        while value > 0 {
            idx -= 1;
            buf[idx] = HEX_LUT[(value & 0xF) as usize];
            value >>= 4;
        }

        &buf[idx..]
    }

    fn i64_to_hex_comp(n: i64, buf: &mut [u8; 16]) -> &[u8] {
        const HEX_LUT: &[u8; 16] = b"0123456789abcdef";
        let mut value = n as u64;
        if value == 0 {
            buf[0] = b'0';
            return &buf[..1];
        }

        // i64最多16个十六进制位
        let mut idx = 16;
        while value > 0 {
            idx -= 1;
            buf[idx] = HEX_LUT[(value & 0xF) as usize];
            value >>= 4;
        }

        &buf[idx..]
    }

    pub fn to_hex(num: i32) -> String {
        let mut buf = [0; 8];
        let slice = Self::i32_to_hex_comp(num, &mut buf);
        std::str::from_utf8(slice).unwrap().to_string()
    }
}