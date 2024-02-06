#[cfg(test)]
mod model_test {
    use crate::tests::*;
    use raylib::prelude::*;

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

    ray_test!(test_load_anims);
    fn test_load_anims(thread: &RaylibThread) {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();

        let _ = rl
            .load_model_animations(&thread, "resources/guy/guyanim.iqm")
            .expect("could not load model animations");
    }

    ray_test!(test_model_from_generated_mesh);
    fn test_model_from_generated_mesh(thread: &RaylibThread) {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();

        let mesh = unsafe { Mesh::gen_mesh_cube(&thread, 1.0, 1.0, 1.0).make_weak() };
        let model = rl.load_model_from_mesh(&thread, mesh).unwrap();

        let zero = Vector3::zero();

        let camera = Camera3D::perspective(zero, zero, zero, 10.0);

        let mut d = rl.begin_drawing(&thread);
        let mut world = d.begin_mode3D(&camera);

        world.draw_model(&model, zero, 1.0, Color::RED);
    }
}
