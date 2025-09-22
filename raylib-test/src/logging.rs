#[cfg(test)]
mod test_logging {
    use crate::tests::*;
    use raylib::prelude::*;

    ray_test!(test_logs);
    fn test_logs(_: &RaylibThread) {
        set_trace_log(TraceLogLevel::LOG_ALL);
        trace_log(TraceLogLevel::LOG_DEBUG, "This Is From `test_logs`");
        set_trace_log(TraceLogLevel::LOG_INFO);
    }
}
