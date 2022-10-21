# Roadmap
1. Replace ffi module with bindgen. 
    - Hand coding ffi gives us more control, but makes it much harder to support multiple architectures which might interpret enums differently (caugh wasm).
    - We can use binggen and build.rs ins raylib-sys to expose all the functions in raylib.
2. Use Argdev's cmake system if we can get cmake to compile to any architecture for any architecture otherwise use the cc crate. 
    - Hoping to support windows, mac, linux, and web at the very least with pi, android, ios, and UWP later.
3. Implement the new architecture. At the very least get core working. 
    - See new architecture. 
4. Bundle in rgui as a feature flag.
    - I think rgui should come in by default since it's cheap, but that's debatable.

# New Architecture
We should take advantage of rust's type system to make UB impossible. We do this mostly through drop already, but it's still possible to do things like start draw before initializing the window. The draw process should look like this

```rust
let drawCtx = handle.start_draw();
// Do the draw
let handle = drawCtx.end_draw();

// Using a texture
let drawCtx = handle.begin_texture_mode(target)
// Do the draw
let (target, handle) = drawCtx.end_draw();
```

This could be done by having two zero sized types RegularDraw and TextureDraw that
implement an unsafe trait RaylibDraw. 

 We'd have to figure out how to keep things thread safe. I'm open to suggestions.


# How safe is raylib
below are all functions currently exposed by raylib. I've maked them safe, safe with specific caveats, and always unsafe.

Safe for a function means you can call it from any thread assuming you have a &RaylibContext or &mut RaylibContext.
Unsafe for a function means you probably can't call it from any thread with out some concurrency guarentees (Often &mut RalibContext will be enough)
A ? means I'm not entirely sure if the function belongs in the category. 

Safe for a struct means the traits Clone and Send/Sync are trivial to implement and a drop implementation is mostly unecessary. 

# Core

## Always Safe - test without any window
GetMonitorCount
GetMonitorWidth
GetMonitorHeight
GetMonitorPhysicalWidth
GetMonitorPhysicalHeight
GetMonitorName
GetClipboardText
SetClipboardText

GetWorldToScreen
GetCameraMatrix

ColorToInt
ColorNormalize
ColorToHSV
ColorFromHSV
GetColor
Fade

SetTraceLogLevel
SetTraceLogExit
SetTraceLogCallback
TraceLog

GetRandomValue

FileExists
IsFileExtension
GetExtension
GetFileName
GetDirectoryPath
GetWorkingDirectory

ChangeDirectory
GetFileModTime

StorageSaveValue
StorageLoadValue

OpenURL

IsKeyPressed
IsKeyDown
IsKeyReleased
IsKeyUp
GetKeyPressed
SetExitKey

IsGamepadAvailable
IsGamepadName
GetGamepadName
IsGamepadButtonPressed
IsGamepadButtonDown
IsGamepadButtonReleased
IsGamepadButtonUp
GetGamepadButtonPressed
GetGamepadAxisCount
GetGamepadAxisMovement

IsMouseButtonPressed
IsMouseButtonDown
IsMouseButtonReleased
IsMouseButtonUp
GetMouseX
GetMouseY
GetMousePosition
SetMousePosition
SetMouseOffset
SetMouseScale
GetMouseWheelMove

GetTouchX
GetTouchY
GetTouchPosition

SetGesturesEnabled
IsGestureDetected
GetGestureDetected
GetTouchPointsCount
GetGestureHoldDuration
GetGestureDragVector
GetGestureDragAngle
GetGesturePinchVector
GetGesturePinchAngle

SetCameraMode
?UpdateCamera - does this require window

SetCameraPanControl
SetCameraAltControl
SetCameraSmoothZoomControl
SetCameraMoveControls

## Safe assuming window is open
?SetConfigFlags - before window is open

WindowShouldClose
IsWindowReady
IsWindowMinimized
IsWindowResized
IsWindowHidden
?ToggleFullscreen - can this be called every frame
?UnhideWindow - safe if not hidden?
?HideWindow
SetWindowIcon
SetWindowTitle
SetWindowPosition
GetScreenWidth
GetScreenHeight

GetMouseRay


ShowCursor
HideCursor
IsCursorHidden
EnableCursor
DisableCursor

?BeginDrawing - can only be called once

SetTargetFPS
GetFPS
GetFrameTime
GetTime

TakeScreenshot

IsFileDropped

## Safe assuming you are in drawing mode
ClearBackground
EndDrawing
BeginMode2D
BeginMode3D
BeginTextureMode

## Safe assuming you are in 2D mode
EndMode2D


## Safe assuming you are in 3D mode
EndMode3D


## Safe assuming you are in texture mode
EndTextureMode


## Safe with prerequisites
InitWindow - can only be called once
CloseWindow - can only be called once
SetWindowMonitor - only works in full screen mode
SetWindowMinSize - only if FLAG_WINDOW_RESIZABLE is set
SetWindowSize - only if FLAG_WINDOW_RESIZABLE is set

GetFileNameWithoutExt - memory should be freed
GetDirectoryFiles && ClearDirectoryFiles - memory should be freed
LoadDroppedFiles && UnloadDroppedFiles - free memory


## Unsafe
GetWindowHandle

## Unsafe and requires memory management

# Shapes

## Always safe
CheckCollisionRecs
CheckCollisionCircles
CheckCollisionCircleRec
GetCollisionRec
CheckCollisionPointRec
CheckCollisionPointCircle
CheckCollisionPointTriangle

## Safe assuming window is open

## Safe assuming drawing is enabled
DrawPixel
DrawPixelV
DrawLine
DrawLineV
DrawLineEx
DrawLineBezier
DrawCircle
DrawCircleSector
DrawCircleSectorLines
DrawCircleGradient
DrawCircleV
DrawCircleLines
DrawRing
DrawRingLines
DrawRectangle
DrawRectangleV
DrawRectangleRec
DrawRectanglePro
DrawRectangleGradientV
DrawRectangleGradientH
DrawRectangleGradientEx
DrawRectangleLines
DrawRectangleLinesEx
DrawRectangleRounded
DrawRectangleRoundedLines
DrawTriangle
DrawTriangleLines
DrawPoly
DrawPolyEx
DrawPolyExLines

?SetShapesTexture - is this a window open op

# Textures

## Always Safe
LoadImage && UnloadImage
LoadImageEx
LoadImagePro
?LoadImageRaw - Feels very unsafe. 
ExportImage
ExportImageAsCode
LoadTexture && UnloadTexture
LoadTextureFromImage
LoadTextureCubemap
LoadRenderTexture
GetPixelDataSize
GetTextureData

ImageCopy
ImageFormat
ImageAlphaMask
ImageAlphaClear
ImageAlphaCrop
ImageAlphaPremultiply
ImageCrop
ImageResize
ImageResizeNN
ImageResizeCanvas
ImageMipmaps
ImageDither
ImageExtractPalette
ImageText
ImageTextEx
ImageFlipVertical
ImageFlipHorizontal
ImageRotateCW
ImageRotateCCW
ImageColorTint
ImageColorInvert
ImageColorGrayscale
ImageColorContrast
ImageColorBrightness
ImageColorReplace

GenImageColor
GenImageGradientV
GenImageGradientH
GenImageGradientRadial
GenImageChecked
GenImageWhiteNoise
GenImagePerlinNoise
GenImageCellular

GenTextureMipmaps
SetTextureFilter
SetTextureWrap

## Safe assuming window is open
GetScreenData


## Safe assuming Drawing is enabled
ImageDraw
ImageDrawRectangle
ImageDrawRectangleLines
ImageDrawText
ImageDrawTextEx

DrawTexture
DrawTextureV
DrawTextureEx
DrawTextureRec
DrawTextureQuad
DrawTexturePro
DrawTextureNPatch

## Unsafe
GetImageData
GetImageDataNormalized
UpdateTexture
ImageToPOT

# Text

## Always Safe
GetFontDefault
LoadFont & UnloadFont
LoadFontEx
LoadFontFromImage
LoadFontData
GenImageFontAtlas

MeasureText
MeasureTextEx
GetGlyphIndex

TextIsEqual
TextLength
TextFormat
TextSubtext
TextReplace
TextInsert
TextJoin
TextSplit
TextAppend
TextFindIndex
TextToUpper
TextToLower
TextToPascal
TextToInteger

## Safe assuming drawing is enabled
DrawFPS
DrawText
DrawTextEx
DrawTextRec
DrawTextRecEx

# Models

## Always Safe
LoadModel && UnloadModel
LoadModelFromMesh

LoadMeshes && UnloadMesh
ExportMesh

LoadMaterials
LoadMaterialDefault
UnloadMaterial
SetMaterialTexture
SetModelMeshMaterial

LoadModelAnimations
UpdateModelAnimation
UnloadModelAnimation
IsModelAnimationValid

GenMeshPoly
GenMeshPlane
GenMeshCube
GenMeshSphere
GenMeshHemiSphere
GenMeshCylinder
GenMeshTorus
GenMeshKnot
GenMeshHeightmap
GenMeshCubicmap

MeshBoundingBox
MeshTangents
MeshBinormals

CheckCollisionSpheres
CheckCollisionBoxes
CheckCollisionBoxSphere
CheckCollisionRaySphere
CheckCollisionRaySphereEx
CheckCollisionRayBox
GetCollisionRayModel
GetCollisionRayTriangle
GetCollisionRayGround

## Safe assuming drawing is enabled
DrawLine3D
DrawCircle3D
DrawCube
DrawCubeV
DrawCubeWires
DrawCubeWiresV
DrawCubeTexture
DrawSphere
DrawSphereEx
DrawSphereWires
DrawCylinder
DrawCylinderWires
DrawPlane
DrawRay
DrawGrid
DrawGizmo

DrawModel
DrawModelEx
DrawModelWires
DrawModelWiresEx
DrawBoundingBox
DrawBillboard
DrawBillboardRec

# Shaders

## Always Safe
LoadText
LoadShader
LoadShaderCode
UnloadShader

GetShaderLocation
SetShaderValue
SetShaderValueV
SetShaderValueMatrix
SetShaderValueTexture
SetMatrixProjection
SetMatrixModelview
GetMatrixModelview

GetShaderDefault
GetTextureDefault

## Assuming drawing
BeginShaderMode
EndShaderMode
BeginBlendMode
EndBlendMode
BeginScissorMode
EndScissorMode

## Asuming VR
InitVrSimulator
CloseVrSimulator
UpdateVrTracking
SetVrConfiguration
IsVrSimulatorReady
ToggleVrMode
BeginVrDrawing
EndVrDrawing

# Audio

## Always Safe
InitAudioDevice
IsAudioDeviceReady

LoadWave
LoadWaveEx
LoadSound
LoadSoundFromWave
UpdateSound
UnloadWave
UnloadSound
ExportWave
ExportWaveAsCode

SetSoundVolume
SetSoundPitch
WaveFormat
WaveCopy
GetWaveData

LoadMusicStream - UnloadMusicStream
UpdateMusicStream
SetMusicVolume
SetMusicPitch
SetMusicLoopCount
GetMusicTimeLength
GetMusicTimePlayed

InitAudioStream
CloseAudioStream
IsAudioBufferProcessed

SetAudioStreamVolume
SetAudioStreamPitch

## Safe assuming audio is initalized
CloseAudioDevice
IsAudioDeviceReady

PlaySound
PauseSound
ResumeSound
StopSound
IsSoundPlaying

PlayMusicStream
StopMusicStream
PauseMusicStream
ResumeMusicStream
IsMusicPlaying

PlayAudioStream
PauseAudioStream
ResumeAudioStream
IsAudioStreamPlaying
StopAudioStream

# Structs 

## Always safe
Vector2
Vector3
Vector4
Quaternion
Matrix
Color
Rectangle

NPatchInfo
CharInfo

Camera
Camera2D
Mesh
MaterialMap

Transform
BoneInfo
?ModelAnimation
Ray
RayHitInfo
BoundingBox


## Safe but require destructors
Image
Texture

RenderTexture
NPatchInfo
Font
Shader
Material
Model