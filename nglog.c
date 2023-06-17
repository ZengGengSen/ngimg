#include <stdarg.h>

#include "nglog.h"

static log_state_t g_log_state;

log_state_t *log_state_get_ptr() {
    return &g_log_state;
}

#define FILE_PATH_LOG_DBG   "[DEBUG]"
#define FILE_PATH_LOG_INFO  "[INFO]"
#define FILE_PATH_LOG_ERROR "[ERROR]"
#define FILE_PATH_LOG_WARN  "[WARN]"

#define LOG_LEVEL_DBG       0
#define LOG_LEVEL_INFO      1
#define LOG_LEVEL_ERROR     2
#define LOG_LEVEL_WARN      3

#define DEFAULT_LOG_LEVEL   LOG_LEVEL_DBG

static void NG_LOG_V(const char *tag, const char *fmt, va_list ap);
#define NG_WARN_V NG_LOG_V
#define NG_ERR_V NG_LOG_V

void NG_LOG_V(const char *tag, const char *fmt, va_list ap) {
    log_state_t *log_st = log_state_get_ptr();
    FILE        *fp     = log_st->fp;
    const char  *tag_v  = tag ? tag : FILE_PATH_LOG_INFO;

    if (fp)
    {
        fprintf(fp, "%s ", tag_v);
        vfprintf(fp, fmt, ap);
        fprintf(fp, "\n");
        fflush(fp);
    }
}

void NG_DBG(const char *fmt, ...) {
   va_list ap;
   log_state_t *log_st = log_state_get_ptr();

   if (!log_st->initialized)
      return;

   if (log_st->log_level > LOG_LEVEL_DBG)
      return;

   va_start(ap, fmt);
   NG_LOG_V(FILE_PATH_LOG_DBG, fmt, ap);
   va_end(ap);
}

void NG_LOG(const char *fmt, ...) {
   va_list ap;
   log_state_t *log_st = log_state_get_ptr();

   if (!log_st->initialized)
      return;
   if (log_st->log_level > 1)
      return;

   va_start(ap, fmt);
   NG_LOG_V(FILE_PATH_LOG_INFO, fmt, ap);
   va_end(ap);
}

void NG_WARN(const char *fmt, ...) {
   va_list ap;
   log_state_t *log_st = log_state_get_ptr();

   if (!log_st->initialized)
      return;
   if (log_st->log_level > 2)
      return;

   va_start(ap, fmt);
   NG_WARN_V(FILE_PATH_LOG_WARN, fmt, ap);
   va_end(ap);
}

void NG_ERR(const char *fmt, ...) {
   va_list ap;
   log_state_t *log_st = log_state_get_ptr();

   if (!log_st->initialized)
      return;

   va_start(ap, fmt);
   NG_ERR_V(FILE_PATH_LOG_ERROR, fmt, ap);
   va_end(ap);
}

void ng_log_init(bool log_to_file) {
    log_state_t *log_st = log_state_get_ptr();

    if (log_to_file) {
        // todo: log_to_file impl
        return;
    } 

    if (log_st->fp != NULL) {
        ng_log_deinit();
    }

    log_st->fp = stderr;
    log_st->buf = NULL;

    log_st->override_path[0] = '\0';
    log_st->initialized = true;
    log_st->log_to_file = false;
    log_st->override_active = false;

    log_st->log_level = DEFAULT_LOG_LEVEL;
}

void ng_log_deinit() {
    log_state_t *log_st = log_state_get_ptr();

    if (!log_st->initialized)
        return;

    if (log_st->log_to_file)
        // todo: add FILE
        ;
    else
        log_st->fp = NULL;

}
