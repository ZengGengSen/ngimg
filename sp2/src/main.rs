#![allow(dead_code, unused_imports)]
use std::fs::File;
use std::io::{self, Seek, Read};
use sp2::palette::Palette;

fn get_all_palette() -> io::Result<()> {
    let path = "C:\\Software\\kof97\\232-p2.sp2";
    let mut file = File::open(path)?;

    let start_offset = 0x2CFFF0;
    let end_offset = 0x2FFFF0;
    let length = end_offset - start_offset;

    file.seek(io::SeekFrom::Start(start_offset))?;

    let mut full_buffer = vec![0; length as usize];
    file.read_exact(&mut full_buffer)?;
    
    let palette_vec: Vec<Palette> = full_buffer
        .chunks_exact(32)
        .map(|chunk| {
            let colors =  Palette::u8x32_to_u16x16_le(chunk.to_vec())?;
            Palette::new_from_vec(colors)
        })
        .collect::<io::Result<Vec<Palette>>>()?
        .try_into()
        .expect("Failed to convert palette");

    for i in 0 .. 15 {
       print!("{:04x} ", palette_vec[0x100].color_array[i]); 
    }

    println!("HelloWorld!");
    Ok(())
}

fn get_all_frame_definition() -> io::Result<()> {
    let path = "C:\\Software\\kof97\\232-p2.sp2";
    let mut file = File::open(path)?;

    let start_offset = 0x100000;
    const BANK_SIZE: usize = 0x100000;

    file.seek(io::SeekFrom::Start(start_offset))?;

    let mut buffer = vec![0; BANK_SIZE];
    file.read_exact(&mut buffer)?;

    buffer.chunks_exact_mut(2)
        .filter(|chunk| chunk.len() == 2)
        .for_each(|chunk| chunk.swap(0, 1));

    let mut offset = 0x5008c;
    for ch_id in 0 .. 0x23 {
        let addr: u32 = u32::from_be_bytes(buffer[offset .. offset + 4].try_into().unwrap());
        offset += 4;

        let size = u16::from_be_bytes(buffer[offset .. offset + 2].try_into().unwrap());
        offset += 2;

        println!("frame_def_ptr: {:08x}, frame_def_size: {:04x}", addr, size);
        for i in 0 .. size {
            let palette_id = buffer[offset];
            offset += 1;

            let draw_type = buffer[offset];
            offset += 1;

            let width = buffer[offset];
            offset += 1;

            let height = buffer[offset];
            offset += 1;

            print!("{:02x} {:03x} {:02x} {:02x} {:02x} {:02x} ", ch_id, i, palette_id, draw_type, width, height);

            match draw_type {
                0x00 | 0x04 => {
                    let tile_base_offset = u32::from_be_bytes(buffer[offset .. offset + 4].try_into().unwrap());
                    offset += 4;

                    print!("{:08x} ", tile_base_offset);
                    for _ in 0 .. width {
                        print!("{:04x} ", u16::from_be_bytes(buffer[offset .. offset + 2].try_into().unwrap()));
                        offset += 2;
                    }
                },
                0x01 => {
                    let tile_base_offset = u32::from_be_bytes(buffer[offset .. offset + 4].try_into().unwrap());
                    offset += 4;

                    print!("{:08x} ", tile_base_offset);
                    for _ in 0 .. width {
                        print!("{:02x} ", buffer[offset]);
                        offset += 1;
                    };

                    if offset & 1 != 0 {
                        print!("{:02x} ", buffer[offset]);
                        offset += 1;
                    };
                },
                0x02 => {
                    for _ in 0 .. width {
                        for _ in 0 .. height {
                            print!("{:02x} {:04x}", buffer[offset], u16::from_be_bytes([buffer[offset + 1], buffer[offset + 2]]));
                            offset += 3;
                        }
                    };

                    if offset & 1 != 0 {
                        print!("{:02x} ", buffer[offset]);
                        offset += 1;
                    };
                },
                0x03 => {
                    let tile_hi_base_offset = u16::from_be_bytes([buffer[offset], buffer[offset + 1]]);
                    offset += 2;
                    print!("    {:04x} ", tile_hi_base_offset);

                    for _ in 0 .. width {
                        for _ in 0 .. height {
                            print!("{:04x} ", u16::from_be_bytes([buffer[offset], buffer[offset + 1]]));
                            offset += 2;
                        }
                    };
                },
                0x05 => {
                    let mut offset_inc = width as usize * 2 + offset;

                    for _ in 0 .. width {
                        let mut mask = u16::from_be_bytes([buffer[offset], buffer[offset + 1]]);
                        offset += 2;

                        print!("{:04x}[ ", mask);

                        let mut swap_toggle = false;
                        for _ in 0 .. height {
                            if mask & 0x8000 == 0x8000 {
                                if swap_toggle {
                                    print!("{:02x} {:04x} ", buffer[offset_inc], u16::from_be_bytes([buffer[offset_inc + 1], buffer[offset_inc + 2]]));
                                } else {
                                    print!("{:04x} {:02x} ", u16::from_be_bytes([buffer[offset_inc], buffer[offset_inc + 1]]), buffer[offset_inc + 2]);
                                };
                                offset_inc += 3;
                            };
                            swap_toggle = !swap_toggle;
                            mask <<= 1;
                        };

                        print!("] ");
                    };

                    offset = offset_inc;

                    if offset & 1 != 0 {
                        print!("({:02x})", buffer[offset]);
                        offset += 1;
                    };
                },
                0x06 => {
                    let mut offset_inc = width as usize + offset;
 
                    if offset_inc & 1 != 0 {
                        offset_inc += 1;
                    };

                    for _ in 0 .. width {
                        let mut mask = buffer[offset];
                        offset += 1;

                        print!("{:02x}[ ", mask);

                        let mut swap_toggle = false;
                        for _ in 0 .. height {
                            if mask & 0x80 == 0x80 {
                                if swap_toggle {
                                    print!("{:02x} {:04x} ", buffer[offset_inc], u16::from_be_bytes([buffer[offset_inc + 1], buffer[offset_inc + 2]]));
                                } else {
                                    print!("{:04x} {:02x} ", u16::from_be_bytes([buffer[offset_inc], buffer[offset_inc + 1]]), buffer[offset_inc + 2]);
                                };
                                offset_inc += 3;
                            };
                            swap_toggle = !swap_toggle;
                            mask <<= 1;
                        };

                        print!("] ");
                    };

                    if offset & 1 != 0 {
                        print!("({:02x})", buffer[offset]);
                    };

                    offset = offset_inc;

                    if offset & 1 != 0 {
                        print!("({:02x})", buffer[offset]);
                        offset += 1;
                    };
                },
                0x07 => {
                    let tile_hi_base_offset = u16::from_be_bytes([buffer[offset], buffer[offset + 1]]);
                    offset += 2;
                    print!("    {:04x} ", tile_hi_base_offset);

                    let mut offset_inc = 2 * width as usize + offset;
                    for _ in 0 .. width {
                        let mut mask = u16::from_be_bytes([buffer[offset], buffer[offset + 1]]);
                        offset += 2;
                        print!("{:04x}[ ", mask);

                        for _ in 0 .. height {
                            if mask & 0x8000 == 0x8000 {
                                print!("{:04x} ", u16::from_be_bytes([buffer[offset_inc], buffer[offset_inc + 1]]));
                                offset_inc += 2;
                            };
                            mask <<= 1;
                        }

                        print!("] ");
                    };

                    offset = offset_inc;
                },
                0x08 => {
                    let tile_hi_base_offset = u16::from_be_bytes(buffer[offset .. offset + 2].try_into().unwrap());
                    offset += 2;
                    print!("    {:04x} ", tile_hi_base_offset);

                    let mut offset_inc = 0;
                    for _ in 0 .. width {
                        let mut mask = buffer[offset];
                        offset += 1;

                        let usize_width = width as usize;

                        print!("{:02x}[ ", mask);

                        for _ in 0 .. height {
                            if mask & 0x80 == 0x80 {
                                print!("{:04x} ", u16::from_be_bytes(buffer[offset + usize_width + offset_inc .. offset + usize_width + offset_inc + 2].try_into().unwrap()));
                                offset_inc += 2;
                            };
                            mask <<= 1;
                        };

                        print!("] ");
                    };

                    if offset & 1 != 0 {
                        print!("{:02x} ", buffer[offset]);
                        offset += 1;
                    };

                    offset += offset_inc;
                },
                0x09 => {
                    let tile_hi_base_offset = u16::from_be_bytes([buffer[offset], buffer[offset + 1]]);
                    offset += 2;
                    print!("    {:04x} ", tile_hi_base_offset);

                    let mut offset_inc = 2 * width as usize + offset;
                    for _ in 0 .. width {
                        let mut mask = u16::from_be_bytes([buffer[offset], buffer[offset + 1]]);
                        offset += 2;
                        print!("{:04x}[ ", mask);

                        for _ in 0 .. height {
                            if mask & 0x8000 == 0x8000 {
                                print!("{:02x} ", buffer[offset_inc]);
                                offset_inc += 1;
                            };
                            mask <<= 1;
                        }

                        print!("] ");
                    };

                    offset = offset_inc;

                    if offset & 1 != 0 {
                        print!("{:02x} ", buffer[offset]);
                        offset += 1;
                    };
                },
                0x0a => {
                    let tile_hi_base_offset = u16::from_be_bytes(buffer[offset .. offset + 2].try_into().unwrap());
                    offset += 2;
                    print!("    {:04x} ", tile_hi_base_offset);

                    let mut offset_inc = 0;
                    for _ in 0 .. width {
                        let mut mask = buffer[offset];
                        offset += 1;

                        let usize_width = width as usize;

                        print!("{:02x}[ ", mask);

                        for _ in 0 .. height {
                            if mask & 0x80 == 0x80 {
                                print!("{:02x} ", buffer[offset + usize_width + offset_inc]);
                                offset_inc += 1;
                            };
                            mask <<= 1;
                        };

                        print!("] ");
                    };

                    offset += offset_inc;

                    if offset & 1 != 0 {
                        print!("{:02x} ", buffer[offset]);
                        offset += 1;
                    };
                },
                _ => return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Unknown draw type {:02x}", draw_type)
                )),
            };
            print!("\n");
        };

        for _ in 0 .. size {
            offset += 4;
        };
    }

    Ok(())
}

fn main() -> io::Result<()> {
    return get_all_frame_definition(); 
    // Ok(())
}

/*const FILE_SIZE: usize = 0x400000;                  // 文件大小为 0x400000 (4MB)
const BANK_NUM: usize = 4;                          // 共4个板块
const BANK_SIZE: usize = FILE_SIZE / BANK_NUM;      // 每个bank的大小为 0x100000 (1MB)

struct Bank {
bank_index: u16,
data: [u8; BANK_SIZE - 2],
}

impl Bank {
fn new() -> Bank {
Bank {
bank_index: 0,
data: [0; BANK_SIZE - 2],
}
} 
}

struct SP2File {
banks: [Bank; BANK_NUM]
}

impl SP2File {
fn from_file(path: &str) -> io::Result<Self> {
let mut file = File::open(path)?;

// 读取文件数据内容
let mut data = Vec::new();
file.read_to_end(&mut data)?;

// 校验文件大小
if data.len() != FILE_SIZE {
return Err(io::Error::new(
io::ErrorKind::InvalidData,
"Invalid file size",
));
}

// 将文件数据分为四个bank
let mut banks: [Bank; 4];

for i in 0..BANK_NUM {
let start = i * BANK_SIZE;
let bank_index = u16::from_le_bytes([data[start], data[start + 1]]);
// banks[i].data.copy_from_slice(&data[start + 2..start + BANK_SIZE]);
banks[i].data.copy_from_slice(&data[start + 2..start + BANK_SIZE]);

let data_slice = &data[start + 2..start + BANK_SIZE];
let owned_data: Vec<u8> = data_slice.to_owned();
banks[i].data.copy_from_slice(&owned_data[..]);

banks[i].bank_index = bank_index.to_be(); // 字节交换，从小端序转换为大端序
}

Ok(SP2File { banks })
}
}

fn main() {
let mut file = SP2File::from_file("D:\\Github\\ngimg\\cmake-build-debug\\232-p2.sp2").unwrap();

printfln!("%02x", file.banks[0].bank_index);
printfln!("%02x", file.banks[1].bank_index);
printfln!("%02x", file.banks[2].bank_index);
printfln!("%02x", file.banks[3].bank_index);
}*/
