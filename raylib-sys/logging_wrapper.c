#include <stdarg.h>
#include <stdio.h>
#include <string.h>

#include "raylib/src/raylib.h"

#define MAX_TRACELOG_MSG_LENGTH 128

typedef void (*callback_t)(int log_level, char *str);

static callback_t trace_log_callback;

void wrapper_trace_log_callback(int log_level, const char *fmt, va_list args) {
    char buffer[MAX_TRACELOG_MSG_LENGTH] = { 0 };
    vsprintf(buffer, fmt, args);
    trace_log_callback(log_level, buffer);
}

void set_trace_log_callback(callback_t c) {
    trace_log_callback = c;
    SetTraceLogCallback(wrapper_trace_log_callback);
}
