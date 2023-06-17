#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "nglog.h"

typedef struct {
    uint16_t draw_method_index;           // $64BC    

    uint16_t data_64B6;
    uint16_t data_64C8;

        
} global_state_t; 

static global_state_t global_state = {};

global_state_t *global_state_get_ptr() { return &global_state; }

typedef struct {
    uint8_t *data;
} frame_state_t;

void draw_func_0();
void draw_func_1();
void draw_func_2();
void draw_func_3();
void draw_func_4();
void draw_func_5();
void draw_func_6();
void draw_func_7();

void draw_func() {
    global_state_t *global_st = global_state_get_ptr();

    typedef void (*draw_func_t)();
    static const draw_func_t draw_func_list[11] = {
        &draw_func_0,
    }; 
    
    draw_func_list[global_st->draw_method_index]();
}

void draw_func_0() {
    global_state_t *global_st = global_state_get_ptr();
    
    // bsr.w   sub_649E
    
    
}

typedef struct {
    uint16_t bank_num;                // bank num, 0x01
    
    // [0x100002, 0x10008D)
    uint32_t frame_xy_address_list[0x23];      // 23 个地址
    
    // [0x0010008E, 0x1272AA)
    struct frame_xy_node {
        uint16_t data[3];
        struct frame_xy_node *next;
    } list[0x23];
    
    // [0x001072AA, 0x00150000), full 0xff
    
    // [0x00150000, 0x150090)
    uint32_t frame_definition_address_list[0x23];      // 23 个地址
    
    struct frame_definition_node {
        uint32_t offset;            // 偏移
        uint16_t size;              // node size
        
        struct {
            uint8_t palette_index;
            uint8_t method_index;
            uint8_t width;
            uint8_t height;
            
            union {
                uint32_t tile_base_num;
                
            } data_0;
        } data;
    };
    
    // [0x001536C6, 0x001540D3)
    
    // [0x0015680E, 
} bank_1_state_t;

typedef struct {
    uint8_t buf[0x100000];

    ;
} bank_state_t;

typedef struct {
    bank_state_t bank_state[4];
} sprite_file_t;

static sprite_file_t g_sprite_file;

void sprite_file_init(const char *path) {
    sprite_file_t *sprite_st = &g_sprite_file;
    
    // 读取文件
    FILE *file;
   
    if (fopen_s(&file, path, "rb") != 0) {
        printf("无法打开文件: %s\n", path);
        return;
    }

    fseek(file, 0, SEEK_END);
    long file_size;
    if (fseek(file, 0, SEEK_END) != 0 || (file_size = ftell(file)) == -1) {
        printf("无法获取文件大小\n");
        fclose(file);
        return;
    }
    fseek(file, 0, SEEK_SET);

    if (file_size < sizeof(sprite_file_t)) {
        printf("文件大小不足以包含完整的sprite_file_t结构\n");
        fclose(file);
        return;
    }
    
    if (fread(sprite_st, sizeof(sprite_file_t), 1, file) != 1) {
        printf("读取文件失败\n");
        fclose(file);
        free(sprite_st);
        return;
    }

    fclose(file);

    for (int i = 0; i < 4; ++i) {
        for (int j = 0; j < 0x50000; ++j) {
            uint8_t tmp = sprite_st->bank_state[i].buf[2 * j];
            sprite_st->bank_state[i].buf[2 * j] = sprite_st->bank_state[i].buf[2 * j + 1];
            sprite_st->bank_state[i].buf[2 * j + 1] = tmp;
        }
    }
}

static inline uint32_t parse_uint32(const uint8_t *data) { return data[0] << 24 | data[1] << 16 | data[2] << 8 | data[3]; }

int main(int argc, char **argv) {
    ng_log_init(false);
    
    sprite_file_init("D:\\Github\\ngimg\\cmake-build-debug\\232-p2.sp2");

    // for (int i = 0; i < 0x23; ++i) {
    //     uint32_t offset = 2 + i * 4;
    //     uint8_t *data = &g_sprite_file.bank_state[1].buf[offset];
    //     uint32_t ch_addr = data[0] << 24 | data[1] << 16 | data[2] << 8 | data[3];
    //     NG_DBG("[Address] %08x\n", ch_addr);
    // }
    
    // [0x150092, 0x1536C6)
    
    uint8_t *bank = g_sprite_file.bank_state[1].buf;
    for (int ch = 0; ch < 0x01; ++ch) {
        uint32_t ch_addr = parse_uint32(bank + 0x50000 + ch * 4) - 0x200000;
        NG_DBG("%08x", ch_addr);
        
        for (int i = 0; i < 642; ++i) {
            uint8_t *start = bank + (parse_uint32(bank + ch_addr + i * 4) - 0x200000);
            char common_msg[PATH_MAX_LENGTH];
            uint8_t *raw_start = start;
            
            uint8_t palette_id = *start++;
            uint8_t draw_method_index = *start++;
            uint8_t width = *start++;
            uint8_t height = *start++;
            
            snprintf(common_msg, sizeof(common_msg), "%02x %02x %02x %02x ", palette_id, draw_method_index, width, height);
            
            switch (draw_method_index) {
                case 0x00:
                case 0x04: {
                    uint32_t tile_num = start[0] << 24 | start[1] << 16 | start[2] << 8 | start[3];
                    start += 4;
                    snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "%08x ", tile_num);
                    
                    for (int j = 0; j < width; ++j) {
                        snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "%04x ", start[0] << 8 | start[1]);
                        start += 2;
                    }
                }
                    break;
                case 0x01: {
                    uint32_t tile_num = start[0] << 24 | start[1] << 16 | start[2] << 8 | start[3];
                    start += 4;
                    snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "%08x ", tile_num);
                    
                    for (int j = 0; j < width; ++j) {
                        snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "%02x ", *start++);
                    }
                    
                    if ((start - raw_start) & 0x01) {
                        snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "(%02x)", *start++);
                    }
                }
                    break;
                case 0x02: {
                    for (int j = 0; j < width; ++j) {
                        for (int k = 0; k < height; ++k) {
                            snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "%04x ", start[0] << 8 | start[1]);
                            start += 2;
                        }
                    }
                }
                    break;
                case 0x03: {
                    uint16_t tile_num = start[0] << 8 | start[1];
                    start += 2;
                    
                    snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "    %04x ", tile_num);
                    
                    for (int j = 0; j < width; ++j) {
                        for (int k = 0; k < height; ++k) {
                            snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "%04x ", start[0] << 8 | start[1]);
                            start += 2;
                        }
                    }
                }
                    break;
                case 0x06: {
                    uint8_t *other_data = start;
                    
                    for (int j = 0; j < width; ++j) {
                        snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "%02x ", *start++);
                    }
                    
                    if (width & 1)
                        snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "(%02x) ", *start++);
                    
                    for (int j = 0; j < width; ++j) {
                        uint16_t tmp = *other_data++;
                        
                        snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "[ ");
                        for (int k = 0; k < height; ++k) {
                            tmp += tmp;
                            if (tmp >= 256) {
                                if (j & 1) {
                                    snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "%02x %04x ", start[0], start[1] << 8 | start[2]);
                                } else {
                                    snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "%04x %02x ", start[0] << 8 | start[1], start[2]);
                                }
                                start+=3;
                            } else {
                            }
                            tmp &= 0xff;
                        }
                        snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "] ");
                    }
                    
                    if ((start - raw_start) & 0x01) {
                        snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "(%02x)", *start++);
                    }
                }
                    break;
                case 0x07: {
                    uint16_t tile_num = start[0] << 8 | start[1];
                    start += 2;
                    
                    uint8_t *other_data = start;
                    snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "    %04x ", tile_num);
                    
                    for (int j = 0; j < width; ++j) {
                        snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "%04x ", start[0] << 8 | start[1]);
                        start+=2;
                    }
                    
                    for (int j = 0; j < width; ++j) {
                        uint32_t tmp = other_data[0] << 8 | other_data[1];
                        other_data += 2;
                        
                        snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "[ ");
                        for (int k = 0; k < height; ++k) {
                            tmp += tmp;
                            if (tmp >= 65536) {
                                snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "%04x ", start[0] << 8 | start[1]);
                                start+=2;
                            } else {
                            }
                            tmp &= 0xFFFF;
                        }
                        snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "] ");
                    }
                }
                    break;
                case 0x08: {
                    uint16_t tile_num = start[0] << 8 | start[1];
                    start += 2;
                    
                    uint8_t *other_data = start;
                    snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "    %04x ", tile_num);
                    
                    for (int j = 0; j < width; ++j) {
                        snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "%02x ", *start++);
                    }
                    
                    if (width & 1)
                        snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "(%02x) ", *start++);
                    
                    for (int j = 0; j < width; ++j) {
                        uint16_t tmp = *other_data++;
                        
                        snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "[ ");
                        for (int k = 0; k < height; ++k) {
                            tmp += tmp;
                            if (tmp >= 256) {
                                snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "%04x ", start[0] << 8 | start[1]);
                                start+=2;
                            } else {
                            }
                            tmp &= 0xff;
                        }
                        snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "] ");
                    }
                }
                    break;
                case 0x09: {
                    uint16_t tile_num = start[0] << 8 | start[1];
                    start += 2;
                    
                    uint8_t *other_data = start;
                    snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "    %04x ", tile_num);
                    
                    for (int j = 0; j < width; ++j) {
                        snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "%04x ", start[0] << 8 | start[1]);
                        start+=2;
                    }
                    
                    for (int j = 0; j < width; ++j) {
                        uint32_t tmp = other_data[0] << 8 | other_data[1];
                        other_data += 2;
                        
                        snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "[ ");
                        for (int k = 0; k < height; ++k) {
                            tmp += tmp;
                            if (tmp >= 65536) {
                                snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "%02x ", *start++);
                            } else {
                            }
                            tmp &= 0xFFFF;
                        }
                        snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "] ");
                    }
                    
                    if ((start - raw_start) & 0x01) {
                        snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "(%02x)", *start++);
                    }
                }
                    break;
                case 0x0A: {
                    uint16_t tile_num = start[0] << 8 | start[1];
                    start += 2;
                    
                    uint8_t *other_data = start;
                    snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "    %04x ", tile_num);
                    
                    for (int j = 0; j < width; ++j) {
                        snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "%02x ", *start++);
                    }
                    
                    for (int j = 0; j < width; ++j) {
                        uint16_t tmp = *other_data++;
                        
                        snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "[ ");
                        for (int k = 0; k < height; ++k) {
                            tmp += tmp;
                            if (tmp >= 256) {
                                snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "%02x ", *start++);
                            } else {
                            }
                            tmp &= 0xff;
                        }
                        snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "] ");
                    }
                    
                    if ((start - raw_start) & 0x01) {
                        snprintf(common_msg + strnlen(common_msg, sizeof(common_msg)), sizeof(common_msg), "(%02x)", *start++);
                    }
                }
                    break;
                default:
                    NG_ERR("[FRAME DEFINITION]: Unknown $%x", draw_method_index);
                    goto end;
            }
            
            NG_DBG("%s", common_msg);
        }
    }
    
end:
    ng_log_deinit();

    return 0;
}
