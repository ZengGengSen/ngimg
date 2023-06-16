#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

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
}

void draw_func_0() {
    
}

typedef struct {
    uint16_t bank_num;                // bank num, 0x01
    uint32_t address_list[0x23];      // 23 个地址
    struct frame_xy_node {
        struct {
            uint32_t d1;
            uint16_t d2;
        } data;
        struct frame_xy_node *next;
    } list;
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

int main(int argc, char **argv) {
    ng_log_init(false);
    
    sprite_file_init("D:\\Github\\ngimg\\cmake-build-debug\\232-p2.sp2");

    for (int i = 0; i < 0x23; ++i) {
        uint32_t offset = 2 + i * 4;
        uint8_t *data = &g_sprite_file.bank_state[1].buf[offset];
        uint32_t ch_addr = data[0] << 24 | data[1] << 16 | data[2] << 8 | data[3];
        NG_DBG("[Address] %08x\n", ch_addr);
    }
    
    ng_log_deinit();

    return 0;
}
