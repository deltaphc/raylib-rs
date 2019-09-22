//! 3D Model, Mesh, and Animation
use crate::core::math::{BoundingBox, Vector3};
use crate::core::texture::Image;
use crate::core::{RaylibHandle, RaylibThread};
use crate::ffi;
use std::ffi::CString;

fn no_drop<T>(_thing: T) {}
make_thin_wrapper!(Model, ffi::Model, ffi::UnloadModel);
make_thin_wrapper!(Mesh, ffi::Mesh, |mut mesh| ffi::UnloadMesh(&mut mesh));
make_thin_wrapper!(Material, ffi::Material, ffi::UnloadMaterial);
make_thin_wrapper!(WeakMaterial, ffi::Material, no_drop);
make_thin_wrapper!(BoneInfo, ffi::BoneInfo, no_drop);
make_thin_wrapper!(
    ModelAnimation,
    ffi::ModelAnimation,
    ffi::UnloadModelAnimation
);
// #[cfg(feature = "nightly")]
// impl !Send for Model {}
// #[cfg(feature = "nightly")]
// unsafe impl Sync for Model {}
// #[cfg(feature = "nightly")]
// impl !Send for Mesh {}
// #[cfg(feature = "nightly")]
// unsafe impl Sync for Mesh {}
// #[cfg(feature = "nightly")]
// impl !Send for Material {}
// #[cfg(feature = "nightly")]
// unsafe impl Sync for Material {}

impl RaylibHandle {
    /// Loads model from files (mesh and material).
    // #[inline]
    pub fn load_model(&mut self, _: &RaylibThread, filename: &str) -> Result<Model, String> {
        let c_filename = CString::new(filename).unwrap();
        let m = unsafe { ffi::LoadModel(c_filename.as_ptr()) };
        if m.meshes.is_null() && m.materials.is_null() && m.bones.is_null() && m.bindPose.is_null()
        {
            return Err(format!("could not load model {}", filename));
        }
        // TODO check if null pointer checks are necessary.
        Ok(Model(m))
    }
}

impl Model {
    pub fn transform(&self) -> &crate::math::Matrix {
        unsafe { std::mem::transmute(&self.0.transform) }
    }

    pub fn set_transform(&mut self, mat: &crate::math::Matrix) {
        self.transform = mat.into();
    }

    pub fn meshes(&self) -> &[Mesh] {
        unsafe {
            std::slice::from_raw_parts(self.0.meshes as *const Mesh, self.0.meshCount as usize)
        }
    }
    pub fn meshes_mut(&mut self) -> &mut [Mesh] {
        unsafe {
            std::slice::from_raw_parts_mut(self.0.meshes as *mut Mesh, self.0.meshCount as usize)
        }
    }
    pub fn materials(&self) -> &[Material] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.materials as *const Material,
                self.0.materialCount as usize,
            )
        }
    }
    pub fn materials_mut(&mut self) -> &mut [Material] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.0.materials as *mut Material,
                self.0.materialCount as usize,
            )
        }
    }

    pub fn bones(&self) -> &[BoneInfo] {
        unsafe {
            std::slice::from_raw_parts(self.0.bones as *const BoneInfo, self.0.boneCount as usize)
        }
    }
    pub fn bones_mut(&mut self) -> &mut [BoneInfo] {
        unsafe {
            std::slice::from_raw_parts_mut(self.0.bones as *mut BoneInfo, self.0.boneCount as usize)
        }
    }
    pub fn bind_pose(&self) -> &crate::math::Transform {
        unsafe { std::mem::transmute(self.0.bindPose) }
    }

    /// Check model animation skeleton match
    #[inline]
    pub fn is_model_animation_valid(&self, anim: &ModelAnimation) -> bool {
        unsafe { ffi::IsModelAnimationValid(self.0, anim.0) }
    }
}

impl RaylibHandle {
    /// Load meshes from model file
    pub fn load_meshes(&mut self, _: &RaylibThread, filename: &str) -> Result<Vec<Mesh>, String> {
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
}

impl Mesh {
    pub fn vertices(&self) -> &[Vector3] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.vertices as *const Vector3,
                self.0.vertexCount as usize,
            )
        }
    }
    pub fn vertices_mut(&mut self) -> &mut [Vector3] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.0.vertices as *mut Vector3,
                self.0.vertexCount as usize,
            )
        }
    }
    pub fn normals(&self) -> &[Vector3] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.normals as *const Vector3,
                self.0.vertexCount as usize,
            )
        }
    }
    pub fn normals_mut(&mut self) -> &mut [Vector3] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.0.normals as *mut Vector3,
                self.0.vertexCount as usize,
            )
        }
    }
    pub fn tangents(&self) -> &[Vector3] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.tangents as *const Vector3,
                self.0.vertexCount as usize,
            )
        }
    }
    pub fn tangents_mut(&mut self) -> &mut [Vector3] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.0.tangents as *mut Vector3,
                self.0.vertexCount as usize,
            )
        }
    }
    pub fn colors(&self) -> &[crate::color::Color] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.colors as *const crate::color::Color,
                self.0.vertexCount as usize,
            )
        }
    }
    pub fn colors_mut(&mut self) -> &mut [crate::color::Color] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.0.colors as *mut crate::color::Color,
                self.0.vertexCount as usize,
            )
        }
    }
    pub fn indicies(&self) -> &[u16] {
        unsafe {
            std::slice::from_raw_parts(self.0.indices as *const u16, self.0.vertexCount as usize)
        }
    }
    pub fn indicies_mut(&mut self) -> &mut [u16] {
        unsafe {
            std::slice::from_raw_parts_mut(self.0.indices as *mut u16, self.0.vertexCount as usize)
        }
    }

    /// Generate polygonal mesh
    #[inline]
    pub fn gen_mesh_poly(_: &RaylibThread, sides: i32, radius: f32) -> Mesh {
        unsafe { Mesh(ffi::GenMeshPoly(sides, radius)) }
    }

    /// Generates plane mesh (with subdivisions).
    #[inline]
    pub fn gen_mesh_plane(
        _: &RaylibThread,
        width: f32,
        length: f32,
        res_x: i32,
        res_z: i32,
    ) -> Mesh {
        unsafe { Mesh(ffi::GenMeshPlane(width, length, res_x, res_z)) }
    }

    /// Generates cuboid mesh.
    #[inline]
    pub fn gen_mesh_cube(_: &RaylibThread, width: f32, height: f32, length: f32) -> Mesh {
        unsafe { Mesh(ffi::GenMeshCube(width, height, length)) }
    }

    /// Generates sphere mesh (standard sphere).
    #[inline]
    pub fn gen_mesh_sphere(_: &RaylibThread, radius: f32, rings: i32, slices: i32) -> Mesh {
        unsafe { Mesh(ffi::GenMeshSphere(radius, rings, slices)) }
    }

    /// Generates half-sphere mesh (no bottom cap).
    #[inline]
    pub fn gen_mesh_hemisphere(_: &RaylibThread, radius: f32, rings: i32, slices: i32) -> Mesh {
        unsafe { Mesh(ffi::GenMeshHemiSphere(radius, rings, slices)) }
    }

    /// Generates cylinder mesh.
    #[inline]
    pub fn gen_mesh_cylinder(_: &RaylibThread, radius: f32, height: f32, slices: i32) -> Mesh {
        unsafe { Mesh(ffi::GenMeshCylinder(radius, height, slices)) }
    }

    /// Generates torus mesh.
    #[inline]
    pub fn gen_mesh_torus(
        _: &RaylibThread,
        radius: f32,
        size: f32,
        rad_seg: i32,
        sides: i32,
    ) -> Mesh {
        unsafe { Mesh(ffi::GenMeshTorus(radius, size, rad_seg, sides)) }
    }

    /// Generates trefoil knot mesh.
    #[inline]
    pub fn gen_mesh_knot(
        _: &RaylibThread,
        radius: f32,
        size: f32,
        rad_seg: i32,
        sides: i32,
    ) -> Mesh {
        unsafe { Mesh(ffi::GenMeshKnot(radius, size, rad_seg, sides)) }
    }

    /// Generates heightmap mesh from image data.
    #[inline]
    pub fn gen_mesh_heightmap(
        _: &RaylibThread,
        heightmap: &Image,
        size: impl Into<ffi::Vector3>,
    ) -> Mesh {
        unsafe { Mesh(ffi::GenMeshHeightmap(heightmap.0, size.into())) }
    }

    /// Generates cubes-based map mesh from image data.
    #[inline]
    pub fn gen_mesh_cubicmap(
        _: &RaylibThread,
        cubicmap: &Image,
        cube_size: impl Into<ffi::Vector3>,
    ) -> Mesh {
        unsafe { Mesh(ffi::GenMeshCubicmap(cubicmap.0, cube_size.into())) }
    }

    /// Computes mesh bounding box limits.
    #[inline]
    pub fn mesh_bounding_box(&self) -> BoundingBox {
        unsafe { ffi::MeshBoundingBox(self.0).into() }
    }

    /// Computes mesh tangents.
    #[inline]
    pub fn mesh_tangents(&mut self) {
        unsafe {
            ffi::MeshTangents(&mut self.0);
        }
    }

    /// Computes mesh binormals.
    #[inline]
    pub fn mesh_binormals(&mut self) {
        unsafe {
            ffi::MeshBinormals(&mut self.0);
        }
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
    pub unsafe fn make_weak(self) -> WeakMaterial {
        let m = WeakMaterial(self.0);
        std::mem::forget(self);
        m
    }

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

    pub fn set_material_texture(
        &mut self,
        map_type: crate::consts::MaterialMapType,
        texture: impl AsRef<ffi::Texture2D>,
    ) {
        unsafe { ffi::SetMaterialTexture(&mut self.0, (map_type as u32) as i32, *texture.as_ref()) }
    }
}

impl RaylibMaterial for WeakMaterial {}
impl RaylibMaterial for Material {}

pub trait RaylibMaterial: AsRef<ffi::Material> + AsMut<ffi::Material> {
    fn shader(&self) -> &crate::shaders::Shader {
        unsafe { std::mem::transmute(&self.as_ref().shader) }
    }

    fn maps(&self) -> &[ffi::MaterialMap] {
        &self.as_ref().maps
    }

    fn maps_mut(&mut self) -> &mut [ffi::MaterialMap] {
        &mut self.as_mut().maps
    }
}

impl ModelAnimation {
    pub fn bones(&self) -> &[BoneInfo] {
        unsafe {
            std::slice::from_raw_parts(self.0.bones as *const BoneInfo, self.0.boneCount as usize)
        }
    }

    pub fn bones_mut(&mut self) -> &mut [BoneInfo] {
        unsafe {
            std::slice::from_raw_parts_mut(self.0.bones as *mut BoneInfo, self.0.boneCount as usize)
        }
    }

    pub fn frame_poses(&self) -> &[&crate::math::Transform] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.framePoses as *const &crate::math::Transform,
                self.0.frameCount as usize,
            )
        }
    }

    pub fn frame_poses_mut(&mut self) -> &mut [&mut crate::math::Transform] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.0.framePoses as *mut &mut crate::math::Transform,
                self.0.frameCount as usize,
            )
        }
    }

    pub fn load_model_animations(filename: &str) -> Result<Vec<ModelAnimation>, String> {
        let c_filename = CString::new(filename).unwrap();
        let mut m_size = 0;
        let m_ptr = unsafe { ffi::LoadModelAnimations(c_filename.as_ptr(), &mut m_size) };
        if m_size <= 0 {
            return Err(format!("No model animations loaded from {}", filename));
        }
        let mut m_vec = Vec::with_capacity(m_size as usize);
        for i in 0..m_size {
            unsafe {
                m_vec.push(ModelAnimation(*m_ptr.offset(i as isize)));
            }
        }
        unsafe {
            libc::free(m_ptr as *mut libc::c_void);
        }
        Ok(m_vec)
    }
}

impl RaylibHandle {
    fn load_material_default(&self, _: &RaylibThread) -> WeakMaterial {
        WeakMaterial(unsafe { ffi::LoadMaterialDefault() })
    }

    /// Weak materials will leak memeory if they are not unlaoded
    /// Unload material from GPU memory (VRAM)
    fn unload_material(&self, _: &RaylibThread, material: WeakMaterial) {
        unsafe { ffi::UnloadMaterial(*material.as_ref()) }
    }
}
