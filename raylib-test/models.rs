
#[cfg(test)]
mod model_test {
    use raylib::prelude::*;
    use crate::tests::*;

    ray_test!(test_load_model);
    fn test_load_model(thread: &RaylibThread) {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();
        let _ = rl.load_model(thread, "resources/cube.obj");
        let _ = rl.load_model(thread, "resources/pbr/trooper.obj");
    }

    ray_test!(test_load_meshes);
    fn test_load_meshes(_thread: &RaylibThread) {
        // TODO run this test when Raysan implements LoadMeshes
        // let m = Mesh::load_meshes(thread, "resources/cube.obj").expect("couldn't load any meshes");
    }

    // ray_test!(test_load_anims);
    #[test]
    fn test_load_anims() {
        let _ = ModelAnimation::load_model_animations("resources/guy/guyanim.iqm")
            .expect("could not load model animations");
    }
}
