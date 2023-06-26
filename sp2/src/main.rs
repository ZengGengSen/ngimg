#![allow(dead_code, unused_imports)]
use std::collections::btree_map::Values;
use std::fs::File;
use std::io::{self, Seek, Read};
use std::{vec, u8, usize};
use sp2::palette::Palette;
use sp2::framedef::FrameDefinition;
use image::{RgbImage, Rgb, GenericImageView, GrayImage, GrayAlphaImage};

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
        println!("{:04x} ", palette_vec[0x100].color_array[i]); 
    }

    println!("HelloWorld!");
    Ok(())
}

fn generate_frame_def(buffer: &[u8], offset: &mut usize) -> io::Result<FrameDefinition> {
    let palette_id = buffer[*offset];
    *offset += 1;

    let draw_type = buffer[*offset];
    *offset += 1;

    let width = buffer[*offset];
    *offset += 1;

    let height = buffer[*offset];
    *offset += 1;

    match draw_type {
        0x00 | 0x04 => {
            let tile_base_offset = u32::from_be_bytes([buffer[*offset], buffer[*offset + 1], buffer[*offset + 2], buffer[*offset + 3]]);
            *offset += 4;

            let bitmap: Vec<u16> = buffer[*offset .. *offset + width as usize * 2].chunks(2).map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]])).collect();
            *offset += width as usize * 2;

            let count = bitmap.iter().fold(0, |acc, &value| acc + value.count_ones()) as usize;
            let tile_offset_list: Vec<u32> = buffer[*offset .. *offset + count].iter().enumerate().map(|(index, _)| tile_base_offset + index as u32).collect();

            Ok(FrameDefinition::new(palette_id, width, height, bitmap, tile_offset_list))
        },
        0x01 => {
            let tile_base_offset = u32::from_be_bytes([buffer[*offset], buffer[*offset + 1], buffer[*offset + 2], buffer[*offset + 3]]);
            *offset += 4;

            let bitmap: Vec<u16> = buffer[*offset .. *offset + width as usize].iter().map(|value| (*value as u16) << 8).collect();
            *offset += width as usize;

            let count = bitmap.iter().fold(0, |acc, &value| acc + value.count_ones()) as usize;
            let tile_offset_list: Vec<u32> = buffer[*offset .. *offset + count].iter().enumerate().map(|(index, _)| tile_base_offset + index as u32).collect();

            if *offset & 1 != 0 { *offset += 1; };

            Ok(FrameDefinition::new(palette_id, width, height, bitmap, tile_offset_list))
        },
        0x02 => {
            let bitmap: Vec<u16> = vec![(((1u32 << (height + 1)) - 1) << (16 - height) & 0xffff) as u16; width as usize]; 

            let count = (width * height) as usize;
            let tile_offset_list: Vec<u32> = buffer[*offset .. *offset + count * 3]
                .chunks(3)
                .map(|chunk| {
                    (chunk[0] as u32) << 12 | u16::from_be_bytes([chunk[1], chunk[2]]) as u32
                })
                .collect();
            *offset += count * 3;

            if *offset & 1 != 0 { *offset += 1; };

            Ok(FrameDefinition::new(palette_id, width, height, bitmap, tile_offset_list))
        },
        0x03 => {
            let tile_hi_base_offset = (u16::from_be_bytes([buffer[*offset], buffer[*offset + 1]]) as u32) << 12;
            *offset += 2;

            println!("{}, {}", width, height);
            let bitmap: Vec<u16> = vec![(((1u32 << (height + 1)) - 1) << (16 - height) & 0xffff) as u16; width as usize]; 

            let count = (width * height) as usize;
            let tile_offset_list: Vec<u32> = buffer[*offset .. *offset + count * 2]
                .chunks(2)
                .map(|chunk| {
                    tile_hi_base_offset + u16::from_be_bytes([chunk[0], chunk[1]]) as u32
                })
                .collect();
            *offset += count * 2;

            Ok(FrameDefinition::new(palette_id, width, height, bitmap, tile_offset_list))
        },
        0x05 => {
            let bitmap: Vec<u16> = buffer[*offset .. *offset + 2 * width as usize]
                .chunks(2)
                .map(|chunk| {
                    u16::from_be_bytes([chunk[0], chunk[1]])
                })
                .collect();
            *offset += 2 * width as usize;

            let count = bitmap.iter().fold(0, |acc, &value| acc + value.count_ones()) as usize;

            let mut swap_tpggle = false;
            let tile_offset_list: Vec<u32> = buffer[*offset .. *offset + count * 3]
                .chunks(3)
                .map(|chunk| {
                    swap_tpggle = !swap_tpggle;
                    if swap_tpggle {
                        let hi = (chunk[0] as u32) << 12;
                        let lo = u16::from_be_bytes([chunk[1], chunk[2]]) as u32;
                        hi | lo
                    } else {
                        let lo = u16::from_be_bytes([chunk[0], chunk[1]]) as u32;
                        let hi = (chunk[2] as u32) << 12;
                        hi | lo
                    }
                })
                .collect();
            *offset += 3 * count;

            if *offset & 1 != 0 { *offset += 1; };

            Ok(FrameDefinition::new(palette_id, width, height, bitmap, tile_offset_list))
        },
        0x06 => {
            let bitmap: Vec<u16> = buffer[*offset .. *offset + width as usize].iter().map(|value| (*value as u16) << 8).collect();
            *offset += width as usize;

            if *offset & 1 != 0 { *offset += 1; };

            let count = bitmap.iter().fold(0, |acc, &value| acc + value.count_ones()) as usize;

            let mut swap_tpggle = false;
            let tile_offset_list: Vec<u32> = buffer[*offset .. *offset + count * 3]
                .chunks(3)
                .map(|chunk| {
                    swap_tpggle = !swap_tpggle;
                    if swap_tpggle {
                        let hi = (chunk[0] as u32) << 12;
                        let lo = u16::from_be_bytes([chunk[1], chunk[2]]) as u32;
                        hi | lo
                    } else {
                        let lo = u16::from_be_bytes([chunk[0], chunk[1]]) as u32;
                        let hi = (chunk[2] as u32) << 12;
                        hi | lo
                    }
                }).collect();
            *offset += 3 * count;

            if *offset & 1 != 0 { *offset += 1; };

            Ok(FrameDefinition::new(palette_id, width, height, bitmap, tile_offset_list))
        },
        0x07 => {
            let tile_hi_base_offset = (u16::from_be_bytes([buffer[*offset], buffer[*offset + 1]]) as u32) << 12;
            *offset += 2;

            let bitmap: Vec<u16> = buffer[*offset .. *offset + 2 * width as usize]
                .chunks(2)
                .map(|chunk| {
                    u16::from_be_bytes([chunk[0], chunk[1]])
                })
                .collect();
            *offset += 2 * width as usize;

            let count = bitmap.iter().fold(0, |acc, &value| acc + value.count_ones()) as usize;
            let tile_offset_list: Vec<u32> = buffer[*offset .. *offset + count * 2]
                .chunks(2)
                .map(|chunk| {
                    tile_hi_base_offset + u16::from_be_bytes([chunk[0], chunk[1]]) as u32
                })
                .collect();
            *offset += 2 * count;

            Ok(FrameDefinition::new(palette_id, width, height, bitmap, tile_offset_list))
        },
        0x08 => {
            let tile_hi_base_offset = (u16::from_be_bytes([buffer[*offset], buffer[*offset + 1]]) as u32) << 12;
            *offset += 2;

            let bitmap: Vec<u16> = buffer[*offset .. *offset + width as usize].iter().map(|value| (*value as u16) << 8).collect();
            *offset += width as usize;

            if *offset & 1 != 0 { *offset += 1; };

            let count = bitmap.iter().fold(0, |acc, &value| acc + value.count_ones()) as usize;
            let tile_offset_list: Vec<u32> = buffer[*offset .. *offset + count * 2].chunks(2).map(|chunk| tile_hi_base_offset + (u16::from_be_bytes([chunk[0], chunk[1]])) as u32).collect();
            *offset += count * 2;

            Ok(FrameDefinition::new(palette_id, width, height, bitmap, tile_offset_list))
        },
        0x09 => {
            let tile_hi_base_offset = (buffer[*offset + 1] as u32) << 12 | (buffer[*offset] as u32) << 8;
            *offset += 2;

            let bitmap: Vec<u16> = buffer[*offset .. *offset + 2 * width as usize]
                .chunks(2)
                .map(|chunk| {
                    u16::from_be_bytes([chunk[0], chunk[1]])
                })
                .collect();
            *offset += 2 * width as usize;

            let count = bitmap.iter().fold(0, |acc, &value| acc + value.count_ones()) as usize;
            let tile_offset_list: Vec<u32> = buffer[*offset .. *offset + count]
                .iter()
                .map(|value| { 
                    tile_hi_base_offset + (*value as u32)
                })
                .collect();
            *offset += count;

            if *offset & 1 != 0 { *offset += 1; };

            Ok(FrameDefinition::new(palette_id, width, height, bitmap, tile_offset_list))
        },
        0x0a => {
            let tile_hi_base_offset = (buffer[*offset + 1] as u32) << 12 | (buffer[*offset] as u32) << 8;
            *offset += 2;

            let bitmap: Vec<u16> = buffer[*offset .. *offset + width as usize].iter().map(|value| (*value as u16) << 8).collect();
            *offset += width as usize;

            let count = bitmap.iter().fold(0, |acc, &value| acc + value.count_ones()) as usize;
            let tile_offset_list: Vec<u32> = buffer[*offset .. *offset + count].iter().map(|value| tile_hi_base_offset + (*value as u32)).collect();
            *offset += count;

            if *offset & 1 != 0 { *offset += 1; };

            Ok(FrameDefinition::new(palette_id, width, height, bitmap, tile_offset_list))
        },
        _ => return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Unknown draw type {:02x}", draw_type)
        )),
    }
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

    let mut ch_frame_def_list = vec![Vec::new(); 0x23];
    for ch_id in 0 .. 0x23 {
        // let addr: u32 = u32::from_be_bytes(buffer[offset .. offset + 4].try_into().unwrap());
        offset += 4;

        let size = u16::from_be_bytes(buffer[offset .. offset + 2].try_into().unwrap());
        offset += 2;

        // println!("frame_def_ptr: {:08x}, frame_def_size: {:04x}", addr, size);
        for _ in 0 .. size {
            // println!("CH_CODE: {:02x}, Index: {}", ch_id, i);

            ch_frame_def_list[ch_id].push(generate_frame_def(&buffer, &mut offset)?);
        };

        for _ in 0 .. size {
            offset += 4;
        };
    }

    ch_frame_def_list.iter().for_each(|ch_frame_def| {
        ch_frame_def.iter().for_each(|frame_def| {
            println!("{}", *frame_def);
        })
    });
    Ok(())
}

struct Tile {
    bitmap: Vec<u8>,
    width: usize,
    height: usize,
}

impl Tile {
    pub fn new(w: usize, h: usize) -> Tile { Tile { bitmap: vec![0; w * h], width: w, height: h } }    
}

struct ROMInfo {
    name: &'static str,
    size: usize
}

macro_rules! rom_infos {
    [$({$name:expr, $size:expr},)*] => {
        [
            $(ROMInfo {
                name: $name,
                size: $size,
            },)*
        ]
    };
}

/**
 * @param raw           一个 0x80 的 u8数据
 * @param data          一个大小为 0x100 的缓存数据
 */
fn new_spr_tile(raw: &[u8]) -> io::Result<Tile> {
    let mut tile = Tile::new(16, 16);

    tile.bitmap.chunks_exact_mut(0x10).enumerate().for_each(|(y, line)| {
        line.chunks_exact_mut(0x08).enumerate().for_each(|(i, half)| {
            let base: usize = (i ^ 1) * 0x40 + (y << 2);
            half.iter_mut().enumerate().for_each(|(x, pixel)| {
                *pixel |= ((raw[3 + base] >> x) & 1) << 3;
                *pixel |= ((raw[1 + base] >> x) & 1) << 2;
                *pixel |= ((raw[2 + base] >> x) & 1) << 1;
                *pixel |= ((raw[0 + base] >> x) & 1) << 0;
            });
        });
    });

    Ok(tile)
}

fn get_all_tile_info() -> io::Result<()> {
    const ROM_INFOS: [ROMInfo; 6] = rom_infos![ 
        {"232-c1.c1", 0x800000},
        {"232-c2.c2", 0x800000},
        {"232-c3.c3", 0x800000},
        {"232-c4.c4", 0x800000},
        {"232-c5.c5", 0x400000},
        {"232-c6.c6", 0x400000},
    ];

    let mut buffer = vec![0; 0x80];
    let mut tiles: Vec<Tile> = Vec::new();

    for info in ROM_INFOS.chunks_exact(2) {
        let mut file1 = File::open(String::from("C:\\Software\\kof97\\") + info[0].name)?; 
        let mut file2 = File::open(String::from("C:\\Software\\kof97\\") + info[1].name)?; 

        let mut buffer1 = vec!(0; info[0].size);
        let mut buffer2 = vec!(0; info[0].size);

        file1.read_exact(&mut buffer1)?;
        file2.read_exact(&mut buffer2)?;

        tiles.extend(buffer1.chunks_exact(0x40).zip(buffer2.chunks_exact(0x40))
            .flat_map(| (b1, b2) | {
                buffer.chunks_exact_mut(2).zip(b1.iter().zip(b2.iter()))
                    .for_each(| (chunk, (bb1, bb2)) | { 
                        chunk[0] = *bb1; 
                        chunk[1] = *bb2;
                    });

                new_spr_tile(&mut buffer)
            }));
    }

    let mut img = GrayAlphaImage::new(16, 16);

    let iter = &mut tiles[0x10001].bitmap.iter();
    for y in 0 .. 16 {
        for x in 0 .. 16 {
            if let Some(num) = iter.next() {
                img.put_pixel(x, y, image::LumaA([*num, if *num != 0 { 255 } else { 0 }]))
            }
        }
    }
    img.save("gray_image.png").unwrap();

    Ok(())
}

fn main() -> io::Result<()> {
    get_all_tile_info() 
    // get_all_frame_definition()
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
