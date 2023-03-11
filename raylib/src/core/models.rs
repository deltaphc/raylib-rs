//! 3D Model, Mesh, and Animation
use crate::core::math::{BoundingBox, Vector3};
use crate::core::texture::Image;
use crate::core::{RaylibHandle, RaylibThread};
use crate::{consts, ffi};
use core::slice;
use std::ffi::CString;

fn no_drop<T>(_thing: T) {}
make_thin_wrapper!(Mesh, ffi::Mesh, |mesh: ffi::Mesh| ffi::UnloadMesh(mesh));
make_bound_thin_wrapper!(Model, ffi::Model, ffi::UnloadModel, RaylibHandle<'bind>);
make_thin_wrapper!(Material, ffi::Material, ffi::UnloadMaterial);
make_thin_wrapper!(BoneInfo, ffi::BoneInfo, no_drop);
make_thin_wrapper!(
    ModelAnimation,
    ffi::ModelAnimation,
    ffi::UnloadModelAnimation
);
make_thin_wrapper!(MaterialMap, ffi::MaterialMap, no_drop);

impl<'bind, 'a> RaylibHandle<'a> {
    /// Loads model from files (mesh and material).
    // #[inline]
    pub fn load_model(&'bind self, _: &RaylibThread, filename: &str) -> Result<Model<'bind, 'a>, String> {
        let c_filename = CString::new(filename).unwrap();
        let m = unsafe { ffi::LoadModel(c_filename.as_ptr()) };
        if m.meshes.is_null() && m.materials.is_null() && m.bones.is_null() && m.bindPose.is_null()
        {
            return Err(format!("could not load model {}", filename));
        }
        // TODO check if null pointer checks are necessary.
        Ok(unsafe { Model::from_raw(m) })
    }

    // Loads model from a generated mesh
    pub fn load_model_from_mesh(
        &'bind self,
        _: &RaylibThread,
        mesh: &Mesh,
    ) -> Result<Model<'bind, 'a>, String> {
        let m = unsafe { ffi::LoadModelFromMesh(mesh.0) };

        if m.meshes.is_null() || m.materials.is_null() {
            return Err("Could not load model from mesh".to_owned());
        }

        Ok(unsafe { Model::from_raw(m) })
    }

    pub fn load_model_animations(
        &self,
        _: &RaylibThread,
        filename: &str,
    ) -> Result<Vec<ModelAnimation>, String> {
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
            ffi::MemFree(m_ptr as *mut libc::c_void);
        }
        Ok(m_vec)
    }

    pub fn update_model_animation(
        &self,
        _: &RaylibThread,
        mut model: impl AsMut<ffi::Model>,
        anim: impl AsRef<ffi::ModelAnimation>,
        frame: i32,
    ) {
        unsafe {
            ffi::UpdateModelAnimation(*model.as_mut(), *anim.as_ref(), frame);
        }
    }
}

impl<'bind, 'a> RaylibModel for Model<'bind, 'a> {}

pub trait RaylibModel: AsRef<ffi::Model> + AsMut<ffi::Model> {
    fn transform(&self) -> &crate::math::Matrix {
        unsafe { std::mem::transmute(&self.as_ref().transform) }
    }

    fn set_transform(&mut self, mat: &crate::math::Matrix) {
        self.as_mut().transform = mat.into();
    }

    fn meshes<'a>(&'a self) -> &'a [Mesh] {
        unsafe {
            slice::from_raw_parts(
                self.as_ref().meshes as *const Mesh,
                self.as_ref().meshCount as usize,
            )
        }
    }

    fn meshes_mut<'a>(&'a mut self) -> &'a mut [Mesh] {
        unsafe {
            slice::from_raw_parts_mut(
                self.as_ref().meshes as *mut Mesh,
                self.as_ref().meshCount as usize,
            )
        }
    }

    fn materials<'a>(&'a self) -> &'a [Material] {
        unsafe {
            slice::from_raw_parts(
                self.as_ref().materials as *const Material,
                self.as_ref().materialCount as usize,
            )
        }
    }

    fn materials_mut<'a>(&'a mut self) -> &'a mut [Material] {
        unsafe {
            slice::from_raw_parts_mut(
                self.as_ref().materials as *mut Material,
                self.as_ref().materialCount as usize,
            )
        }
    }

    fn bones<'a>(&'a self) -> Option<&'a [BoneInfo]> {
        if self.as_ref().bones.is_null() {
            return None;
        }

        Some(unsafe {
            slice::from_raw_parts(
                self.as_ref().bones as *const BoneInfo,
                self.as_ref().boneCount as usize,
            )
        })
    }

    fn bones_mut<'a>(&'a mut self) -> Option<&'a mut [BoneInfo]> {
        if self.as_ref().bones.is_null() {
            return None;
        }

        Some(unsafe {
            slice::from_raw_parts_mut(
                self.as_ref().bones as *mut BoneInfo,
                self.as_ref().boneCount as usize,
            )
        })
    }

    fn bind_pose<'a>(&'a self) -> Option<&'a crate::math::Transform> {
        if self.as_ref().bindPose.is_null() {
            return None;
        }

        Some(unsafe { std::mem::transmute(self.as_ref().bindPose) })
    }

    fn bind_pose_mut<'a>(&'a mut self) -> Option<&'a mut crate::math::Transform> {
        if self.as_ref().bindPose.is_null() {
            return None;
        }

        Some(unsafe { std::mem::transmute(self.as_mut().bindPose) })
    }

    /// Check model animation skeleton match
    #[inline]
    fn is_model_animation_valid(&self, anim: &ModelAnimation) -> bool {
        unsafe { ffi::IsModelAnimationValid(*self.as_ref(), anim.0) }
    }
}

impl RaylibMesh for Mesh {}

pub trait RaylibMesh: AsRef<ffi::Mesh> + AsMut<ffi::Mesh> {
    fn vertices<'a>(&'a self) -> &'a [Vector3] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_ref().vertices as *const Vector3,
                self.as_ref().vertexCount as usize,
            )
        }
    }
    fn vertices_mut<'a>(&'a mut self) -> &'a mut [Vector3] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.as_mut().vertices as *mut Vector3,
                self.as_mut().vertexCount as usize,
            )
        }
    }
    fn normals<'a>(&'a self) -> &'a [Vector3] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_ref().normals as *const Vector3,
                self.as_ref().vertexCount as usize,
            )
        }
    }
    fn normals_mut<'a>(&'a mut self) -> &'a mut [Vector3] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.as_mut().normals as *mut Vector3,
                self.as_mut().vertexCount as usize,
            )
        }
    }
    fn tangents<'a>(&'a self) -> &'a [Vector3] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_ref().tangents as *const Vector3,
                self.as_ref().vertexCount as usize,
            )
        }
    }
    fn tangents_mut<'a>(&'a mut self) -> &'a mut [Vector3] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.as_mut().tangents as *mut Vector3,
                self.as_mut().vertexCount as usize,
            )
        }
    }
    fn colors<'a>(&'a self) -> &'a [crate::color::Color] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_ref().colors as *const crate::color::Color,
                self.as_ref().vertexCount as usize,
            )
        }
    }
    fn colors_mut<'a>(&'a mut self) -> &'a mut [crate::color::Color] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.as_mut().colors as *mut crate::color::Color,
                self.as_mut().vertexCount as usize,
            )
        }
    }
    fn indicies<'a>(&'a self) -> &'a [u16] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_ref().indices as *const u16,
                self.as_ref().vertexCount as usize,
            )
        }
    }
    fn indicies_mut<'a>(&'a mut self) -> &'a mut [u16] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.as_mut().indices as *mut u16,
                self.as_mut().vertexCount as usize,
            )
        }
    }

    /// Generate polygonal mesh
    #[inline]
    fn gen_mesh_poly(_: &RaylibThread, sides: i32, radius: f32) -> Mesh {
        unsafe { Mesh(ffi::GenMeshPoly(sides, radius)) }
    }

    /// Generates plane mesh (with subdivisions).
    #[inline]
    fn gen_mesh_plane(_: &RaylibThread, width: f32, length: f32, res_x: i32, res_z: i32) -> Mesh {
        unsafe { Mesh(ffi::GenMeshPlane(width, length, res_x, res_z)) }
    }

    /// Generates cuboid mesh.
    #[inline]
    fn gen_mesh_cube(_: &RaylibThread, width: f32, height: f32, length: f32) -> Mesh {
        unsafe { Mesh(ffi::GenMeshCube(width, height, length)) }
    }

    /// Generates sphere mesh (standard sphere).
    #[inline]
    fn gen_mesh_sphere(_: &RaylibThread, radius: f32, rings: i32, slices: i32) -> Mesh {
        unsafe { Mesh(ffi::GenMeshSphere(radius, rings, slices)) }
    }

    /// Generates half-sphere mesh (no bottom cap).
    #[inline]
    fn gen_mesh_hemisphere(_: &RaylibThread, radius: f32, rings: i32, slices: i32) -> Mesh {
        unsafe { Mesh(ffi::GenMeshHemiSphere(radius, rings, slices)) }
    }

    /// Generates cylinder mesh.
    #[inline]
    fn gen_mesh_cylinder(_: &RaylibThread, radius: f32, height: f32, slices: i32) -> Mesh {
        unsafe { Mesh(ffi::GenMeshCylinder(radius, height, slices)) }
    }

    /// Generates torus mesh.
    #[inline]
    fn gen_mesh_torus(_: &RaylibThread, radius: f32, size: f32, rad_seg: i32, sides: i32) -> Mesh {
        unsafe { Mesh(ffi::GenMeshTorus(radius, size, rad_seg, sides)) }
    }

    /// Generates trefoil knot mesh.
    #[inline]
    fn gen_mesh_knot(_: &RaylibThread, radius: f32, size: f32, rad_seg: i32, sides: i32) -> Mesh {
        unsafe { Mesh(ffi::GenMeshKnot(radius, size, rad_seg, sides)) }
    }

    /// Generates heightmap mesh from image data.
    #[inline]
    fn gen_mesh_heightmap(
        _: &RaylibThread,
        heightmap: &Image,
        size: impl Into<ffi::Vector3>,
    ) -> Mesh {
        unsafe { Mesh(ffi::GenMeshHeightmap(heightmap.0, size.into())) }
    }

    /// Generates cubes-based map mesh from image data.
    #[inline]
    fn gen_mesh_cubicmap(
        _: &RaylibThread,
        cubicmap: &Image,
        cube_size: impl Into<ffi::Vector3>,
    ) -> Mesh {
        unsafe { Mesh(ffi::GenMeshCubicmap(cubicmap.0, cube_size.into())) }
    }

    /// Computes mesh bounding box limits.
    #[inline]
    fn get_mesh_bounding_box(&self) -> BoundingBox {
        unsafe { ffi::GetMeshBoundingBox(*self.as_ref()).into() }
    }

    /// Computes mesh tangents.
    // NOTE: New VBO for tangents is generated at default location and also binded to mesh VAO
    #[inline]
    fn gen_mesh_tangents(&mut self, _: &RaylibThread) {
        unsafe {
            ffi::GenMeshTangents(self.as_mut());
        }
    }

    /// Exports mesh as an OBJ file.
    #[inline]
    fn export_mesh(&self, filename: &str) {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            ffi::ExportMesh(*self.as_ref(), c_filename.as_ptr());
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
            ffi::MemFree(m_ptr as *mut libc::c_void);
        }
        Ok(m_vec)
    }
}

impl RaylibMaterial for Material {}

pub trait RaylibMaterial: AsRef<ffi::Material> + AsMut<ffi::Material> {
    fn shader<'a>(&'a self) -> &'a crate::shaders::Shader {
        unsafe { std::mem::transmute(&self.as_ref().shader) }
    }

    fn shader_mut<'a>(&'a mut self) -> &'a mut crate::shaders::Shader {
        unsafe { std::mem::transmute(&mut self.as_mut().shader) }
    }

    fn maps<'a>(&'a self) -> &'a [MaterialMap] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_ref().maps as *const MaterialMap,
                consts::MAX_MATERIAL_MAPS as usize,
            )
        }
    }

    fn maps_mut<'a>(&'a mut self) -> &'a mut [MaterialMap] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.as_mut().maps as *mut MaterialMap,
                consts::MAX_MATERIAL_MAPS as usize,
            )
        }
    }

    fn set_material_texture(
        &mut self,
        map_type: crate::consts::MaterialMapIndex,
        texture: impl AsRef<ffi::Texture2D>,
    ) {
        unsafe {
            ffi::SetMaterialTexture(self.as_mut(), (map_type as u32) as i32, *texture.as_ref())
        }
    }
}

impl RaylibModelAnimation for ModelAnimation {}

pub trait RaylibModelAnimation: AsRef<ffi::ModelAnimation> + AsMut<ffi::ModelAnimation> {
    fn bones<'a>(&'a self) -> &'a [BoneInfo] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_ref().bones as *const BoneInfo,
                self.as_ref().boneCount as usize,
            )
        }
    }

    fn bones_mut<'a>(&'a mut self) -> &'a mut [BoneInfo] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.as_mut().bones as *mut BoneInfo,
                self.as_mut().boneCount as usize,
            )
        }
    }

    fn frame_poses<'a>(&'a self) -> Vec<&'a [crate::math::Transform]> {
        let anim = self.as_ref();

        let mut top = Vec::with_capacity(anim.frameCount as usize);

        for i in 0..anim.frameCount {
            top.push(unsafe {
                std::slice::from_raw_parts(
                    *(anim.framePoses.offset(i as isize) as *const *const crate::math::Transform),
                    anim.boneCount as usize,
                )
            });
        }

        top
    }

    fn frame_poses_mut<'a>(&'a mut self) -> Vec<&'a mut [crate::math::Transform]> {
        let anim = self.as_ref();
        let mut top = Vec::with_capacity(anim.frameCount as usize);

        for i in 0..anim.frameCount {
            top.push(unsafe {
                std::slice::from_raw_parts_mut(
                    *(anim.framePoses.offset(i as isize) as *mut *mut crate::math::Transform),
                    anim.boneCount as usize,
                )
            });
        }

        top
    }
}

impl MaterialMap {
    pub fn texture<'a>(&'a self) -> &'a crate::texture::Texture2D {
        unsafe { std::mem::transmute(&self.0.texture) }
    }
    pub fn texture_mut<'a>(&'a mut self) -> &'a mut crate::texture::Texture2D {
        unsafe { std::mem::transmute(&mut self.0.texture) }
    }

    pub fn color<'a>(&'a self) -> &'a crate::color::Color {
        unsafe { std::mem::transmute(&self.0.color) }
    }
    pub fn color_mut<'a>(&'a mut self) -> &'a mut crate::color::Color {
        unsafe { std::mem::transmute(&mut self.0.color) }
    }

    pub fn value<'a>(&'a self) -> &'a f32 {
        unsafe { std::mem::transmute(&self.0.value) }
    }
    pub fn value_mut<'a>(&'a mut self) -> &'a mut f32 {
        unsafe { std::mem::transmute(&mut self.0.value) }
    }
}

impl<'bind> RaylibHandle<'bind> {
    pub fn load_material_default(&self, _: &RaylibThread) -> Material {
        Material(unsafe { ffi::LoadMaterialDefault() })
    }
}
