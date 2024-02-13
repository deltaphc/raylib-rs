#[cfg(test)]
mod test_logging {
    use crate::tests::*;
    use raylib::prelude::*;

    ray_test!(test_logs);
    fn test_logs(_: &RaylibThread) {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();
        rl.set_trace_log(TraceLogLevel::LOG_ALL);
        rl.trace_log(TraceLogLevel::LOG_DEBUG, "This Is From `test_logs`");
        rl.set_trace_log(TraceLogLevel::LOG_INFO);
    }
}
