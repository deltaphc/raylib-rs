#[cfg(test)]
mod core_test {
    
    use crate::tests::*;
    

    #[test]
    fn test_storage() {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();
        rl.storage_save_value(0, 5);
        let val = rl.storage_load_value(0);
        assert_eq!(val, 5, "storage load read a different value from store");
        rl.storage_save_value(1, 6);
        let val = rl.storage_load_value(1);
        assert_eq!(val, 6, "storage load read a different value from store");
    }
}
