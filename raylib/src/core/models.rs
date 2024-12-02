//! 3D Model, Mesh, and Animation

use crate::core::math::{BoundingBox, Vector3};
use crate::core::texture::Image;
use crate::core::{RaylibHandle, RaylibThread};
use crate::error::{error, Error};
use crate::{consts, ffi};
use std::ffi::CString;
use std::os::raw::c_void;

fn no_drop<T>(_thing: T) {}
make_thin_wrapper!(Model, ffi::Model, ffi::UnloadModel);
make_thin_wrapper!(WeakModel, ffi::Model, no_drop);
make_thin_wrapper!(Mesh, ffi::Mesh, |mesh: ffi::Mesh| ffi::UnloadMesh(mesh));
make_thin_wrapper!(WeakMesh, ffi::Mesh, no_drop);
make_thin_wrapper!(Material, ffi::Material, ffi::UnloadMaterial);
make_thin_wrapper!(WeakMaterial, ffi::Material, no_drop);
make_thin_wrapper!(BoneInfo, ffi::BoneInfo, no_drop);
make_thin_wrapper!(
    ModelAnimation,
    ffi::ModelAnimation,
    ffi::UnloadModelAnimation
);
make_thin_wrapper!(WeakModelAnimation, ffi::ModelAnimation, no_drop);
make_thin_wrapper!(MaterialMap, ffi::MaterialMap, no_drop);

// Weak things can be clone
impl Clone for WeakModel {
    fn clone(&self) -> WeakModel {
        WeakModel(self.0)
    }
}

// Weak things can be clone
impl Clone for WeakMesh {
    fn clone(&self) -> WeakMesh {
        WeakMesh(self.0)
    }
}

// Weak things can be clone
impl Clone for WeakMaterial {
    fn clone(&self) -> WeakMaterial {
        WeakMaterial(self.0)
    }
}

// Weak things can be clone
impl Clone for WeakModelAnimation {
    fn clone(&self) -> WeakModelAnimation {
        WeakModelAnimation(self.0)
    }
}

impl RaylibHandle {
    /// Loads model from files (mesh and material).
    // #[inline]
    pub fn load_model(&mut self, _: &RaylibThread, filename: &str) -> Result<Model, Error> {
        let c_filename = CString::new(filename).unwrap();
        let m = unsafe { ffi::LoadModel(c_filename.as_ptr()) };
        if m.meshes.is_null() && m.materials.is_null() && m.bones.is_null() && m.bindPose.is_null()
        {
            return Err(error!("could not load model", filename));
        }
        // TODO check if null pointer checks are necessary.
        Ok(Model(m))
    }

    // Loads model from a generated mesh
    pub fn load_model_from_mesh(
        &mut self,
        _: &RaylibThread,
        mesh: WeakMesh,
    ) -> Result<Model, Error> {
        let m = unsafe { ffi::LoadModelFromMesh(mesh.0) };

        if m.meshes.is_null() || m.materials.is_null() {
            return Err(error!("Could not load model from mesh"));
        }

        Ok(Model(m))
    }

    pub fn load_model_animations(
        &mut self,
        _: &RaylibThread,
        filename: &str,
    ) -> Result<Vec<ModelAnimation>, Error> {
        let c_filename = CString::new(filename).unwrap();
        let mut m_size = 0;
        let m_ptr = unsafe { ffi::LoadModelAnimations(c_filename.as_ptr(), &mut m_size) };
        if m_size <= 0 {
            return Err(error!("No model animations loaded", filename));
        }
        let mut m_vec = Vec::with_capacity(m_size as usize);
        for i in 0..m_size {
            unsafe {
                m_vec.push(ModelAnimation(*m_ptr.offset(i as isize)));
            }
        }
        unsafe {
            ffi::UnloadModelAnimations(m_ptr, m_size);
        }
        Ok(m_vec)
    }

    pub fn update_model_animation(
        &mut self,
        _: &RaylibThread,
        mut model: impl AsMut<ffi::Model>,
        anim: impl AsRef<ffi::ModelAnimation>,
        frame: i32,
    ) {
        unsafe {
            ffi::UpdateModelAnimation(*model.as_mut(), *anim.as_ref(), frame);
        }
    }

    pub fn update_model_animation_bones(
        &mut self,
        _: &RaylibThread,
        mut model: impl AsMut<ffi::Model>,
        anim: impl AsRef<ffi::ModelAnimation>,
        frame: i32,
    ) {
        unsafe {
            ffi::UpdateModelAnimationBones(*model.as_mut(), *anim.as_ref(), frame);
        }
    }
}

impl RaylibModel for WeakModel {}
impl RaylibModel for Model {}

impl Model {
    pub unsafe fn make_weak(self) -> WeakModel {
        let m = WeakModel(self.0);
        std::mem::forget(self);
        m
    }
}

pub trait RaylibModel: AsRef<ffi::Model> + AsMut<ffi::Model> {
    fn transform(&self) -> &crate::math::Matrix {
        unsafe { std::mem::transmute(&self.as_ref().transform) }
    }

    fn set_transform(&mut self, mat: &crate::math::Matrix) {
        self.as_mut().transform = mat.into();
    }

    fn meshes(&self) -> &[WeakMesh] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_ref().meshes as *const WeakMesh,
                self.as_ref().meshCount as usize,
            )
        }
    }
    fn meshes_mut(&mut self) -> &mut [WeakMesh] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.as_mut().meshes as *mut WeakMesh,
                self.as_mut().meshCount as usize,
            )
        }
    }
    fn materials(&self) -> &[WeakMaterial] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_ref().materials as *const WeakMaterial,
                self.as_ref().materialCount as usize,
            )
        }
    }
    fn materials_mut(&mut self) -> &mut [WeakMaterial] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.as_mut().materials as *mut WeakMaterial,
                self.as_mut().materialCount as usize,
            )
        }
    }

    fn bones(&self) -> Option<&[BoneInfo]> {
        if self.as_ref().bones.is_null() {
            return None;
        }

        Some(unsafe {
            std::slice::from_raw_parts(
                self.as_ref().bones as *const BoneInfo,
                self.as_ref().boneCount as usize,
            )
        })
    }
    fn bones_mut(&mut self) -> Option<&mut [BoneInfo]> {
        if self.as_ref().bones.is_null() {
            return None;
        }

        Some(unsafe {
            std::slice::from_raw_parts_mut(
                self.as_mut().bones as *mut BoneInfo,
                self.as_mut().boneCount as usize,
            )
        })
    }
    fn bind_pose(&self) -> Option<&crate::math::Transform> {
        if self.as_ref().bindPose.is_null() {
            return None;
        }
        Some(unsafe { std::mem::transmute(self.as_ref().bindPose) })
    }

    fn bind_pose_mut(&mut self) -> Option<&mut crate::math::Transform> {
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

    /// Check if a model is ready
    fn is_model_valid(&self) -> bool {
        unsafe { ffi::IsModelValid(*self.as_ref()) }
    }

    /// Compute model bounding box limits (considers all meshes)
    fn get_model_bounding_box(&self) -> BoundingBox {
        unsafe { BoundingBox::from(ffi::GetModelBoundingBox(*self.as_ref())) }
    }

    /// Set material for a mesh
    fn set_model_mesh_material(&mut self, mesh_id: i32, material_id: i32) -> Result<(), Error> {
        if mesh_id >= self.as_ref().meshCount {
            return Err(error!("mesh_id greater than mesh count"));
        } else if material_id >= self.as_ref().materialCount {
            return Err(error!("material_id greater than material count"));
        } else {
            unsafe { ffi::SetModelMeshMaterial(self.as_mut(), mesh_id, material_id) };
            return Ok(());
        };
    }
}

impl RaylibMesh for WeakMesh {}
impl RaylibMesh for Mesh {}

impl Mesh {
    pub unsafe fn make_weak(self) -> WeakMesh {
        let m = WeakMesh(self.0);
        std::mem::forget(self);
        m
    }
}
pub trait RaylibMesh: AsRef<ffi::Mesh> + AsMut<ffi::Mesh> {
    unsafe fn upload(&mut self, dynamic: bool) {
        ffi::UploadMesh(self.as_mut(), dynamic);
    }
    unsafe fn update_buffer<A>(&mut self, index: i32, data: &[u8], offset: i32) {
        ffi::UpdateMeshBuffer(
            *self.as_ref(),
            index,
            data.as_ptr() as *const c_void,
            data.len() as i32,
            offset,
        );
    }
    fn vertices(&self) -> &[Vector3] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_ref().vertices as *const Vector3,
                self.as_ref().vertexCount as usize,
            )
        }
    }
    fn vertices_mut(&mut self) -> &mut [Vector3] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.as_mut().vertices as *mut Vector3,
                self.as_mut().vertexCount as usize,
            )
        }
    }
    fn normals(&self) -> &[Vector3] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_ref().normals as *const Vector3,
                self.as_ref().vertexCount as usize,
            )
        }
    }
    fn normals_mut(&mut self) -> &mut [Vector3] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.as_mut().normals as *mut Vector3,
                self.as_mut().vertexCount as usize,
            )
        }
    }
    fn tangents(&self) -> &[Vector3] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_ref().tangents as *const Vector3,
                self.as_ref().vertexCount as usize,
            )
        }
    }
    fn tangents_mut(&mut self) -> &mut [Vector3] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.as_mut().tangents as *mut Vector3,
                self.as_mut().vertexCount as usize,
            )
        }
    }
    fn colors(&self) -> &[crate::color::Color] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_ref().colors as *const crate::color::Color,
                self.as_ref().vertexCount as usize,
            )
        }
    }
    fn colors_mut(&mut self) -> &mut [crate::color::Color] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.as_mut().colors as *mut crate::color::Color,
                self.as_mut().vertexCount as usize,
            )
        }
    }
    fn indicies(&self) -> &[u16] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_ref().indices as *const u16,
                self.as_ref().vertexCount as usize,
            )
        }
    }
    fn indicies_mut(&mut self) -> &mut [u16] {
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

    /// Generate cone/pyramid mesh
    fn gen_mesh_cone(_: &RaylibThread, radius: f32, height: f32, slices: i32) -> Mesh {
        unsafe { Mesh(ffi::GenMeshCone(radius, height, slices)) }
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
    fn export(&self, filename: &str) {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            ffi::ExportMesh(*self.as_ref(), c_filename.as_ptr());
        }
    }

    /// Export mesh as code file (.h) defining multiple arrays of vertex attributes
    #[inline]
    fn export_as_code(&self, filename: &str) {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            ffi::ExportMeshAsCode(*self.as_ref(), c_filename.as_ptr());
        }
    }
}

impl Material {
    pub unsafe fn make_weak(self) -> WeakMaterial {
        let m = WeakMaterial(self.0);
        std::mem::forget(self);
        m
    }

    pub fn load_materials(filename: &str) -> Result<Vec<Material>, Error> {
        let c_filename = CString::new(filename).unwrap();
        let mut m_size = 0;
        let m_ptr = unsafe { ffi::LoadMaterials(c_filename.as_ptr(), &mut m_size) };
        if m_size <= 0 {
            return Err(error!("No materials loaded", filename));
        }
        let mut m_vec = Vec::with_capacity(m_size as usize);
        for i in 0..m_size {
            unsafe {
                m_vec.push(Material(*m_ptr.offset(i as isize)));
            }
        }
        unsafe {
            ffi::MemFree(m_ptr as *mut ::std::os::raw::c_void);
        }
        Ok(m_vec)
    }
}

impl RaylibMaterial for WeakMaterial {}
impl RaylibMaterial for Material {}

pub trait RaylibMaterial: AsRef<ffi::Material> + AsMut<ffi::Material> {
    fn shader(&self) -> &crate::shaders::WeakShader {
        unsafe { std::mem::transmute(&self.as_ref().shader) }
    }

    fn shader_mut(&mut self) -> &mut crate::shaders::WeakShader {
        unsafe { std::mem::transmute(&mut self.as_mut().shader) }
    }

    fn maps(&self) -> &[MaterialMap] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_ref().maps as *const MaterialMap,
                consts::MAX_MATERIAL_MAPS as usize,
            )
        }
    }

    fn maps_mut(&mut self) -> &mut [MaterialMap] {
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

    fn is_material_valid(&mut self) -> bool {
        unsafe { ffi::IsMaterialValid(*self.as_ref()) }
    }
}

impl RaylibModelAnimation for ModelAnimation {}
impl RaylibModelAnimation for WeakModelAnimation {}

impl ModelAnimation {
    pub unsafe fn make_weak(self) -> WeakModelAnimation {
        let m = WeakModelAnimation(self.0);
        std::mem::forget(self);
        m
    }
}

pub trait RaylibModelAnimation: AsRef<ffi::ModelAnimation> + AsMut<ffi::ModelAnimation> {
    fn bones(&self) -> &[BoneInfo] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_ref().bones as *const BoneInfo,
                self.as_ref().boneCount as usize,
            )
        }
    }

    fn bones_mut(&mut self) -> &mut [BoneInfo] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.as_mut().bones as *mut BoneInfo,
                self.as_mut().boneCount as usize,
            )
        }
    }

    fn frame_poses(&self) -> Vec<&[crate::math::Transform]> {
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

    fn frame_poses_mut(&mut self) -> Vec<&mut [crate::math::Transform]> {
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
    pub fn texture(&self) -> &crate::texture::WeakTexture2D {
        unsafe { std::mem::transmute(&self.0.texture) }
    }
    pub fn texture_mut(&mut self) -> &mut crate::texture::WeakTexture2D {
        unsafe { std::mem::transmute(&mut self.0.texture) }
    }

    pub fn color(&self) -> &crate::color::Color {
        unsafe { std::mem::transmute(&self.0.color) }
    }
    pub fn color_mut(&mut self) -> &mut crate::color::Color {
        unsafe { std::mem::transmute(&mut self.0.color) }
    }

    pub fn value(&self) -> &f32 {
        unsafe { std::mem::transmute(&self.0.value) }
    }
    pub fn value_mut(&mut self) -> &mut f32 {
        unsafe { std::mem::transmute(&mut self.0.value) }
    }
}

impl RaylibHandle {
    pub fn load_material_default(&self, _: &RaylibThread) -> WeakMaterial {
        WeakMaterial(unsafe { ffi::LoadMaterialDefault() })
    }

    /// Weak materials will leak memeory if they are not unlaoded
    /// Unload material from GPU memory (VRAM)
    pub unsafe fn unload_material(&mut self, _: &RaylibThread, material: WeakMaterial) {
        {
            ffi::UnloadMaterial(*material.as_ref())
        }
    }

    /// Weak models will leak memeory if they are not unlaoded
    /// Unload model from GPU memory (VRAM)
    pub unsafe fn unload_model(&mut self, _: &RaylibThread, model: WeakModel) {
        {
            ffi::UnloadModel(*model.as_ref())
        }
    }

    /// Weak model_animations will leak memeory if they are not unlaoded
    /// Unload model_animation from GPU memory (VRAM)
    pub unsafe fn unload_model_animation(
        &mut self,
        _: &RaylibThread,
        model_animation: WeakModelAnimation,
    ) {
        {
            ffi::UnloadModelAnimation(*model_animation.as_ref())
        }
    }

    /// Weak meshs will leak memeory if they are not unlaoded
    /// Unload mesh from GPU memory (VRAM)
    pub unsafe fn unload_mesh(&mut self, _: &RaylibThread, mesh: WeakMesh) {
        {
            ffi::UnloadMesh(*mesh.as_ref())
        }
    }
}
