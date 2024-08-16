# raylib-rs Changelog

## 3.7.0 (WIP)

- [core] ADDED: LoadVrStereoConfig()
- [core] ADDED: UnloadVrStereoConfig()
- [core] ADDED: BeginVrStereoMode()
- [core] ADDED: EndVrStereoMode()

- [core] ADDED: GetCurrentMonitor() (#1485) by @object71
  [core] ADDED: SetGamepadMappings() (#1506)
- [core] RENAMED: struct Camera: camera.type to camera.projection
- [core] RENAMED: LoadShaderCode() to LoadShaderFromMemory() (#1690)
- [core] RENAMED: SetMatrixProjection() to rlSetMatrixProjection()
- [core] RENAMED: SetMatrixModelview() to rlSetMatrixModelview()
- [core] RENAMED: GetMatrixModelview() to rlGetMatrixModelview()
- [core] RENAMED: GetMatrixProjection() to rlGetMatrixProjection()
- [core] RENAMED: GetShaderDefault() to rlGetShaderDefault()
  [core] RENAMED: GetTextureDefault() to rlGetTextureDefault()
  [core] REMOVED: GetShapesTexture()
  [core] REMOVED: GetShapesTextureRec()
  [core] REMOVED: GetMouseCursor()
- [core] REMOVED: SetTraceLogExit()
  [core] REVIEWED: GetFileName() and GetDirectoryPath() (#1534) by @gilzoide
  [core] REVIEWED: Wait() to support FreeBSD (#1618)
  [core] REVIEWED: HighDPI support on macOS retina (#1510)
  [core] REDESIGNED: GetFileExtension(), includes the .dot
  [core] REDESIGNED: IsFileExtension(), includes the .dot
  [core] REDESIGNED: Compresion API to use sdefl/sinfl libs
- [rlgl] ADDED: SUPPORT_GL_DETAILS_INFO config flag
  [rlgl] REMOVED: GenTexture\*() functions (#721)
  [rlgl] REVIEWED: rlLoadShaderDefault()
  [rlgl] REDESIGNED: rlLoadExtensions(), more details exposed
  [raymath] REVIEWED: QuaternionFromEuler() (#1651)
  [raymath] REVIEWED: MatrixRotateZYX() (#1642)
- [shapes] ADDED: DrawLineBezierQuad() (#1468) by @epsilon-phase
  [shapes] ADDED: CheckCollisionLines()
- [shapes] ADDED: CheckCollisionPointLine() by @mkupiec1
  [shapes] REVIEWED: CheckCollisionPointTriangle() by @mkupiec1
  [shapes] REDESIGNED: SetShapesTexture()
- [shapes] REDESIGNED: DrawCircleSector(), to use float params
- [shapes] REDESIGNED: DrawCircleSectorLines(), to use float params
- [shapes] REDESIGNED: DrawRing(), to use float params
- [shapes] REDESIGNED: DrawRingLines(), to use float params
- [textures] ADDED: DrawTexturePoly() and example (#1677) by @chriscamacho
- [textures] ADDED: UnloadImageColors() for allocs consistency
  [textures] RENAMED: GetImageData() to LoadImageColors()
  [textures] REVIEWED: ImageClearBackground() and ImageDrawRectangleRec() (#1487) by @JeffM2501
  [textures] REVIEWED: DrawTexturePro() and DrawRectanglePro() transformations (#1632) by @ChrisDill
  [text] REDESIGNED: DrawFPS()
- [models] ADDED: UploadMesh() (#1529)
  [models] ADDED: UpdateMeshBuffer()
  [models] ADDED: DrawMesh()
  [models] ADDED: DrawMeshInstanced()
  [models] ADDED: UnloadModelAnimations() (#1648) by @object71
  :( [models] REMOVED: DrawGizmo()
- [models] REMOVED: LoadMeshes()
  [models] REMOVED: MeshNormalsSmooth()
- [models] REVIEWED: DrawLine3D() (#1643)
  [audio] REVIEWED: Multichannel sound system (#1548)
  [audio] REVIEWED: jar_xm library (#1701) by @jmorel33
  [utils] ADDED: SetLoadFileDataCallback()
  [utils] ADDED: SetSaveFileDataCallback()
  [utils] ADDED: SetLoadFileTextCallback()
  [utils] ADDED: SetSaveFileTextCallback()
  [examples] ADDED: text_draw_3d (#1689) by @Demizdor
  [examples] ADDED: textures_poly (#1677) by @chriscamacho
  [examples] ADDED: models_gltf_model (#1551) by @object71
  [examples] RENAMED: shaders_rlgl_mesh_instanced to shaders_mesh_intancing
  [examples] REDESIGNED: shaders_rlgl_mesh_instanced by @moliad
  [examples] REDESIGNED: core_vr_simulator
  [examples] REDESIGNED: models_yaw_pitch_roll
  [build] ADDED: Config flag: SUPPORT_STANDARD_FILEIO
  [build] ADDED: Config flag: SUPPORT_WINMM_HIGHRES_TIMER (#1641)
  [build] ADDED: Config flag: SUPPORT_GL_DETAILS_INFO
  [build] ADDED: Examples projects to VS2019 solution
  [build] REVIEWED: Makefile to support PLATFORM_RPI (#1580)
  [build] REVIEWED: Multiple typecast warnings by @JeffM2501
  [build] REDESIGNED: VS2019 project build paths
  [build] REDESIGNED: CMake build system by @object71
  [*] RENAMED: Several functions parameters for consistency
  [*] UPDATED: Multiple bindings to latest version
  [*] UPDATED: All external libraries to latest versions
  [*] Multiple code improvements and fixes by multiple contributors!

## 3.5.0 (Done)

Added: SetWindowState
Added: ClearW‌indowState
Added: IsWindowFocused
Added: GetWindowScaleDPI
Added: GetMonitorRefreshRate
Added: IsCursorOnScreen
Added: SetMouseCursor/GetMouseCursor
Added: Normalize
Added: Remap
Added: Vector2Reflect
Added: Vector2LengthSqr
Added: Vector2MoveTowards
Added: UnloadFontData
Added: LoadFontFromMemmory(ttf)
Added: ColorAlphaBlend
Added: GetPixelColor
Added: SetPixelColor
Added: LoadImageFromMemory
Added: LoadImageAnim
Added: DrawTextureTiled
Added: UpdateTextureRec
Added: UnloadImageColors,
Added: UnloadImagePallet,
Added: UnloadWaveSample
Added: DrawTriangle3D
Added: DrawTriangleStrip3D
Added: LoadWaveFromMemory
Added: MemAlloc() / MemFree()
Added: UnloadFileData
Added: UnloadFileText

## 0.10.0 (WIP)

- Basic macOS support. Currently untested.
- Improved ergonomics across the board:
  - Copied over and tweaked many FFI structs so that fields use proper types instead of FFI types.
  - Added `vec2`, `vec3`, `quat`, `rgb`, and `rgba` convenience functions for a middle ground between `From` conversion and `new` methods.
  - Changed several key and gamepad functions to use `u32`, making it more ergonomic with key/gamepad constants.
  - Added optional `prelude` module for conveniently bringing in all the common types and definitions.
- Fixed unnecessary `&mut` in `load_image_ex` and `draw_poly_ex`.
- Fixed linking on MSVC toolchains by including `user32`.
- Prevent `RaylibHandle` from being manually constructed. Fixes a safety soundness hole.

## 0.9.1

- Fixed docs.rs build by removing use of a uniform module path. This also keeps the crate compatible with Rust 1.31+.

## 0.9.0

- Initial crates.io release
