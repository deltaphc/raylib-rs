#if defined(__cplusplus)
extern "C"
{
#endif

#if !defined(RAYGUI_STANDALONE)
#include "../raylib/src/raylib.h"
#endif

#include "utils_log.h"
#include <stdio.h>	// Required for: vprintf()
#include <string.h> // Required for: strcpy(), strcat()

#define MAX_TRACELOG_BUFFER_SIZE 128 // As defined in utils.c from raylib

	void rayLogWrapperCallback(int logType, const char *text, va_list args)
	{
		char buffer[MAX_TRACELOG_BUFFER_SIZE] = {0};

		vsnprintf(buffer, MAX_TRACELOG_BUFFER_SIZE, text, args);

		custom_trace_log_callback(logType, buffer);
	}

	void setLogCallbackWrapper(void)
	{
		SetTraceLogCallback(rayLogWrapperCallback);
	}

#if defined(__cplusplus)
}
#endif
