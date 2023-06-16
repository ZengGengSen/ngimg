#ifndef NGLOG_H
#define NGLOG_H

#include "types.h"

typedef struct {
    FILE *fp;
    void *buf;

    char override_path[PATH_MAX_LENGTH];
    bool initialized;
    bool log_to_file;
    bool override_active;

    uint32_t log_level;
} log_state_t;

#ifdef __cplusplus
extern "C" {
#endif

log_state_t *log_state_get_ptr();

void ng_log_init(bool log_to_file);
void ng_log_deinit();

void NG_DBG(const char *fmt, ...);
void NG_LOG(const char *fmt, ...);
void NG_WARN(const char *fmt, ...);
void NG_ERR(const char *fmt, ...);

#ifdef __cplusplus
}
#endif

#endif // NGLOG_H
