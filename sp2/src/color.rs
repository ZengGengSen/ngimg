pub enum ColorMode {
    RGB666NeoGeo(u16)
}

const NG_COLOR_VALS: [u8; 64] = [
    // Dark,Bright
    0x00,0x00,    //--00
    0x08,0x08,    //--01
    0x0e,0x0e,    //--02
    0x15,0x16,    //--03
    0x1e,0x1e,    //--04
    0x26,0x26,    //--05
    0x2c,0x2c,    //--06
    0x33,0x34,    //--07
    0x40,0x41,    //--08
    0x47,0x49,    //--09
    0x4d,0x4f,    //--0a
    0x55,0x56,    //--0b
    0x5e,0x5f,    //--0c
    0x65,0x67,    //--0d
    0x6b,0x6d,    //--0e
    0x73,0x75,    //--0f
    0x88,0x8a,    //--10
    0x90,0x92,    //--11
    0x96,0x98,    //--12
    0x9e,0xa0,    //--13
    0xa6,0xa9,    //--14
    0xae,0xb0,    //--15
    0xb4,0xb6,    //--16
    0xbc,0xbe,    //--17
    0xc8,0xcb,    //--18
    0xd0,0xd3,    //--19
    0xd6,0xd9,    //--1a
    0xdd,0xe1,    //--1b
    0xe6,0xe9,    //--1c
    0xee,0xf1,    //--1d
    0xf4,0xf7,    //--1e
    0xfb,0xff     //--1f
];

fn conv_8_to_neogeo(in_col: u8) -> usize {
    for i in 0 .. NG_COLOR_VALS.len() {
        if NG_COLOR_VALS[i] >= in_col {
            return i;
        }
    };

    0
}

fn is_rgb666neogeo_dark(r: usize, g: usize, b: usize) -> bool {
    let mut r_even = r / 2 == 0;
    let mut g_even = g / 2 == 0;
    let mut b_even = b / 2 == 0;

    if r_even && g_even && b_even {
        r_even = NG_COLOR_VALS[r] != NG_COLOR_VALS[r + 1];
        g_even = NG_COLOR_VALS[g] != NG_COLOR_VALS[g + 1];
        b_even = NG_COLOR_VALS[b] != NG_COLOR_VALS[b + 1];
    };

    r_even && g_even && b_even
}

pub fn conv_rgb666neogeo_to_argb8888(in_col: u16) -> u32 {
    let darkbit = (in_col >> 0xf) & 0x01;
    let red1 = ((in_col >> 0xe) & 0x01) * 2;
    let redm = ((in_col >> 0x8) & 0x0f) * 4;
    let green1 = ((in_col >> 0xd) & 0x01) * 2;
    let greenm = ((in_col >> 0x4) & 0x0f) * 4;
    let blue1 = ((in_col >> 0xc) & 0x01) * 2;
    let bluem = ((in_col >> 0x0) & 0x0f) * 4;
    let auxa = 0xFF;

    let blue: u32 = NG_COLOR_VALS[(((blue1 + bluem) - darkbit) + 1) as usize] as u32;
    let green: u32 = NG_COLOR_VALS[(((green1 + greenm) - darkbit) + 1) as usize] as u32;
    let red: u32 = NG_COLOR_VALS[(((red1 + redm) - darkbit) + 1) as usize] as u32;

    (auxa << 24) | (blue << 16) | (green << 8) | (red << 0)
}

pub fn conv_argb8888_to_rgb666neogeo(in_col: u32) -> u16 {
    let auxb = (in_col & 0x00FF0000) >> 16;
    let auxg = (in_col & 0x0000FF00) >> 8;
    let auxr = in_col & 0x000000FF;

    let red = conv_8_to_neogeo(auxr as u8);
    let green = conv_8_to_neogeo(auxg as u8);
    let blue = conv_8_to_neogeo(auxb as u8);

    // todo: add dark bit transfer
    // let darkbit: u16 = (if is_rgb666neogeo_dark(red, green, blue) { 0x1 } else { 0x0 }) << 0xf;

    let red1 = (((red / 2) & 0x1) << 0xe) as u16;
    let red_main = (((red / 4) & 0xf) << 0x8) as u16;
    let green1 = (((green / 2) & 0x1) << 0xd) as u16;
    let green_main = (((green / 4) & 0xf) << 0x4) as u16;
    let blue1 = (((blue / 2) & 0x1) << 0xc) as u16;
    let blue_main = (((blue / 4) & 0xf) << 0x0) as u16;

    red1 | red_main | green1 | green_main | blue1 | blue_main
}

pub fn conv_to_argb8888(col: ColorMode) -> u32 {
    match col {
        ColorMode::RGB666NeoGeo(in_col) => conv_rgb666neogeo_to_argb8888(in_col),
    }
}

#[cfg(test)]
mod test {
}
