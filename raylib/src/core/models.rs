use crate::core::*;
use crate::ffi;

make_thin_wrapper!(Model, ffi::Model, ffi::UnloadModel);
make_thin_wrapper!(Mesh, ffi::Mesh, |mut mesh| ffi::UnloadMesh(&mut mesh));
make_thin_wrapper!(Material, ffi::Material, ffi::UnloadMaterial);

impl !Send for Model {}
unsafe impl Sync for Model {}
impl !Send for Mesh {}
unsafe impl Sync for Mesh {}
impl !Send for Material {}
unsafe impl Sync for Material {}

impl Model {
    /// Loads model from files (mesh and material).
    #[inline]
    pub fn load_model(_: &RaylibThread, filename: &str) -> Model {
        let c_filename = CString::new(filename).unwrap();
        unsafe { Model(ffi::LoadModel(c_filename.as_ptr())) }
    }
}

impl Mesh {
    /// Load meshes from model file
    #[inline]
    pub fn load_meshes(_: &RaylibThread, filename: &str) -> Result<Vec<Mesh>, String> {
        let c_filename = CString::new(filename).unwrap();
        let mut m_size = 0;
        let m_ptr = unsafe { ffi::LoadMeshes(c_filename.as_ptr(), &mut m_size) };
        if m_size <= 0 {
            return Err(format!("No meshes loaded from {}", filename));
        }
        let mut m_vec = Vec::with_capacity(m_size as usize);
        for i in 0..m_size {
            unsafe {
                m_vec.push(Mesh(*m_ptr.offset(i as isize)));
            }
        }
        unsafe {
            libc::free(m_ptr as *mut libc::c_void);
        }
        Ok(m_vec)
    }

    /// Exports mesh as an OBJ file.
    #[inline]
    pub fn export_mesh(&self, filename: &str) {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            ffi::ExportMesh(self.0, c_filename.as_ptr());
        }
    }
}

impl Material {
    pub fn load_materials(filename: &str) -> Result<Vec<Material>, String> {
        let c_filename = CString::new(filename).unwrap();
        let mut m_size = 0;
        let m_ptr = unsafe { ffi::LoadMaterials(c_filename.as_ptr(), &mut m_size) };
        if m_size <= 0 {
            return Err(format!("No materials loaded from {}", filename));
        }
        let mut m_vec = Vec::with_capacity(m_size as usize);
        for i in 0..m_size {
            unsafe {
                m_vec.push(Material(*m_ptr.offset(i as isize)));
            }
        }
        unsafe {
            libc::free(m_ptr as *mut libc::c_void);
        }
        Ok(m_vec)
    }
}

#[cfg(test)]
mod model_test {
    use crate::core::*;
    use crate::tests::*;

    ray_test!(test_load_model);
    fn test_load_model(thread: &RaylibThread) {
        let _ = Model::load_model(thread, "resources/cube.obj");
        let _ = Model::load_model(thread, "resources/pbr/trooper.obj");
    }

    ray_test!(test_load_meshes);
    fn test_load_meshes(_thread: &RaylibThread) {
        // TODO run this test when Raysan implements LoadMeshes
        // let m = Mesh::load_meshes(thread, "resources/cube.obj").expect("couldn't load any meshes");
    }
}
