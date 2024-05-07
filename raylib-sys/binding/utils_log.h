#if defined(__cplusplus)
extern "C"
{ // Prevents name mangling of functions
#endif

    void setLogCallbackWrapper(void); // enable the call-back
    void custom_trace_log_callback(int logType, const char *text);

#if defined(__cplusplus)
}
#endif
