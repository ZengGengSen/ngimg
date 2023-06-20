use core::fmt;
use std::fmt::write;

#[derive(Clone)]
pub struct FrameDefinition {
    palette: u8,
    width: u8,
    height: u8,
    bitmap: Vec<u16>,
    tile_offset: Vec<u32>,
}

impl fmt::Display for FrameDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Frame Definition:")?;
        writeln!(f, "Palette: {}", self.palette)?;
        writeln!(f, "Width: {}", self.width)?;
        writeln!(f, "Height: {}", self.height)?;
        for i in 0 .. self.height {
            let bit_mask = 1 << (15 - i);

            for j in 0 .. self.width as usize {
                let mask = self.bitmap[j];

                if mask & bit_mask == bit_mask {
                    write!(f, "1")?;
                } else {
                    write!(f, "0")?;
                }
            }
            write!(f, "\n")?;
        }
        // writeln!(f, "Bitmap: {:04x?}", self.bitmap)?;
        writeln!(f, "Tile Offset: {:05x?}", self.tile_offset)?;
        Ok(())
    }
}

impl FrameDefinition {
    pub fn new(palette: u8, width: u8, height: u8, bitmap: Vec<u16>, tile_offset: Vec<u32>) -> FrameDefinition {
        FrameDefinition {
            palette, width, height, bitmap, tile_offset,
        }
    }
}

