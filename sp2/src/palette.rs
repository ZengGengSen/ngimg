use std::io;

pub struct Palette {
    pub palette_index: u16,
    pub color_array: Vec<u16>,
}

impl Palette {
    pub fn new_from_vec(arr: Vec<u16>) -> io::Result<Palette> {
        if arr.len() != 16 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid vector length. Expected length: 16",
            ));
        }
        let palette_index = arr[0];
        let color_array = arr[1..].to_vec();
        Ok(Palette {
            palette_index,
            color_array,
        })
    }

    pub fn new_from_vec_idx(idx: u16, arr: Vec<u16>) -> io::Result<Palette> {
        if arr.len() != 15 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid vector length. Expected length: 16",
            ));
        }
        Ok(Palette {
            palette_index: idx,
            color_array: arr.to_vec(),
        })
    }

    pub fn u8x32_to_u16x16_le(buf: Vec<u8>) -> io::Result<Vec<u16>> {
        if buf.len() != 32 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid vector length. Expected length: 32",
            ));
        }

        let result: Vec<u16> = buf 
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect::<Vec<u16>>()
            .try_into()
            .expect("Failed to convert");

        Ok(result)
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palette_u8_to_u16() {
        let data: [u8; 32] = [
            0x00, 0x00, 0xFF, 0x7F, 0x00, 0x00, 0xB4, 0x5F, 0xA2, 0x5F, 0x80, 0x6F, 0x70, 0x4F, 0x50, 0x2F,
            0x40, 0x4E, 0x30, 0x4B, 0x20, 0x48, 0x0C, 0x10, 0x09, 0x00, 0xDD, 0x0D, 0xBB, 0x0B, 0x99, 0x09,
        ];

        let expect_result: [u16; 16] = [
            0x0000, 0x7FFF, 0x0000, 0x5FB4, 0x5FA2, 0x6F80, 0x4F70, 0x2F50,
            0x4E40, 0x4B30, 0x4820, 0x100C, 0x0009, 0x0DDD, 0x0BBB, 0x0999,
        ];

        let result: [u16; 16] = data
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect::<Vec<u16>>()
            .try_into()
            .expect("Failed to convert");

        assert_eq!(result, expect_result);
    }

    #[test]
    fn test_palette_new_form_arr() {
        let result = [
            0x0000, 0x7FFF, 0x0000, 0x5FB4, 0x5FA2, 0x6F80, 0x4F70, 0x2F50,
            0x4E40, 0x4B30, 0x4820, 0x100C, 0x0009, 0x0DDD, 0x0BBB, 0x0999,
        ];
        let palette = Palette::new_from_vec(result.to_vec()).unwrap();

        assert_eq!(palette.palette_index, 0);
        assert_eq!(palette.color_array, result[1..]);
    }

    #[test]
    fn test_palette_new_from_vec() {
        let expect_result: [u16; 16] = [
            0x0000, 0x7FFF, 0x0000, 0x5FB4, 0x5FA2, 0x6F80, 0x4F70, 0x2F50,
            0x4E40, 0x4B30, 0x4820, 0x100C, 0x0009, 0x0DDD, 0x0BBB, 0x0999,
        ];

        let palette = Palette::new_from_vec_idx(expect_result[0], expect_result[1..].to_vec()).unwrap();

        assert_eq!(palette.palette_index, 0);
        assert_eq!(palette.color_array, expect_result[1..].to_vec());
    }
}
