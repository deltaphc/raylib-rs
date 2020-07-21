/*******************************************************************************************
*
*   raylib [rlgl] example - Using rlgl module as standalone module
*
*   NOTE: This example requires OpenGL 3.3 or ES2 versions for shaders support,
*         OpenGL 1.1 does not support shaders but it can also be used.
*
*   DEPENDENCIES:
*       rlgl.h    - OpenGL 1.1 immediate-mode style coding translation layer
*       glad.h    - OpenGL extensions initialization library (required by rlgl)
*       raymath.h - 3D math library (required by rlgl)
*       glfw3     - Windows and context initialization library
*
*   rlgl library is provided as a single-file header-only library, this library
*   allows coding in a pseudo-OpenGL 1.1 style while translating calls to multiple
*   OpenGL versions backends (1.1, 2.1, 3.3, ES 2.0).
*
*   COMPILATION:
*       gcc -o rlgl_standalone.exe rlgl_standalone.c -s -Iexternal\include -I..\..\src  \
*           -L. -Lexternal\lib -lglfw3 -lopengl32 -lgdi32 -Wall -std=c99 -DGRAPHICS_API_OPENGL_33
*
*   LICENSE: zlib/libpng
*
*   This example is licensed under an unmodified zlib/libpng license, which is an OSI-certified,
*   BSD-like license that allows static linking with closed source software:
*
*   Copyright (c) 2014-2019 Ramon Santamaria (@raysan5)
*
*   This software is provided "as-is", without any express or implied warranty. In no event
*   will the authors be held liable for any damages arising from the use of this software.
*
*   Permission is granted to anyone to use this software for any purpose, including commercial
*   applications, and to alter it and redistribute it freely, subject to the following restrictions:
*
*     1. The origin of this software must not be misrepresented; you must not claim that you
*     wrote the original software. If you use this software in a product, an acknowledgment
*     in the product documentation would be appreciated but is not required.
*
*     2. Altered source versions must be plainly marked as such, and must not be misrepresented
*     as being the original software.
*
*     3. This notice may not be removed or altered from any source distribution.
*
********************************************************************************************/

use raylib::ffi;
use raylib::prelude::*;

//----------------------------------------------------------------------------------
// Main Entry point
//----------------------------------------------------------------------------------
pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) {
    unsafe {
        // Initialization
        //--------------------------------------------------------------------------------------
        let screenWidth = 800;
        let screenHeight = 450;

        rl.set_window_size(screenWidth, screenHeight);
        let window = rl.get_window_handle();

        // // GLFW3 Initialization + OpenGL 3.3 Context + Extensions
        // //--------------------------------------------------------
        // glfwSetErrorCallback(ErrorCallback);

        // if (!glfwInit())
        // {
        //     printf("GLFW3: Can not initialize GLFW\n");
        //     return 1;
        // }
        // else printf("GLFW3: GLFW initialized successfully\n");

        // glfwWindowHint(GLFW_SAMPLES, 4);
        // glfwWindowHint(GLFW_DEPTH_BITS, 16);
        // glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
        // glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
        // glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
        // //glfwWindowHint(GLFW_OPENGL_DEBUG_CONTEXT, GL_TRUE);

        // GLFWwindow *window = glfwCreateWindow(screenWidth, screenHeight, "rlgl standalone", NULL, NULL);

        // if (!window)
        // {
        //     glfwTerminate();
        //     return 2;
        // }
        // else printf("GLFW3: Window created successfully\n");

        // glfwSetWindowPos(window, 200, 200);

        // glfwSetKeyCallback(window, KeyCallback);

        // glfwMakeContextCurrent(window);
        // glfwSwapInterval(0);

        // Load OpenGL 3.3 supported extensions
        // ffi::rlLoadExtensions(glfwGetProcAddress);
        //--------------------------------------------------------

        // Initialize OpenGL context (states and resources)
        // rlglInit(screenWidth, screenHeight);

        // Initialize viewport and internal projection/modelview matrices
        // rlViewport(0, 0, screenWidth, screenHeight);
        ffi::rlMatrixMode(ffi::RL_PROJECTION as i32); // Switch to PROJECTION matrix
        ffi::rlLoadIdentity(); // Reset current matrix (PROJECTION)
        ffi::rlOrtho(0.0, screenWidth as f64, screenHeight as f64, 0.0, 0.0, 1.0); // Orthographic projection with top-left corner at (0,0)
        ffi::rlMatrixMode(ffi::RL_MODELVIEW as i32); // Switch back to MODELVIEW matrix
        ffi::rlLoadIdentity(); // Reset current matrix (MODELVIEW)

        ffi::rlClearColor(245, 245, 245, 255); // Define clear color
        ffi::rlEnableDepthTest(); // Enable DEPTH_TEST for 3D

        let camera =
            Camera3D::perspective(rvec3(5.0, 5.0, 5.0), Vector3::zero(), Vector3::up(), 45.0);

        let cubePosition = Vector3::zero(); // Cube default position (center)
                                            //--------------------------------------------------------------------------------------

        // Main game loop
        while (!rl.window_should_close()) {
            let d = rl.begin_drawing(&thread);
            // Update
            //----------------------------------------------------------------------------------
            //camera.position.x += 0.01f;
            //----------------------------------------------------------------------------------

            // Draw
            //----------------------------------------------------------------------------------
            ffi::rlClearScreenBuffers(); // Clear current framebuffer

            // Draw '3D' elements in the scene
            //-----------------------------------------------
            // Calculate projection matrix (from perspective) and view matrix from camera look at
            let matProj = Matrix::perspective(
                camera.fovy.to_radians(),
                screenWidth as f32 / screenHeight as f32,
                0.01,
                1000.0,
            );
            let matView = Matrix::look_at(camera.position, camera.target, camera.up);

            ffi::SetMatrixModelview(matView.into()); // Set internal modelview matrix (default shader)
            ffi::SetMatrixProjection(matProj.into()); // Set internal projection matrix (default shader)

            DrawCube(cubePosition, 2.0, 2.0, 2.0, Color::RED);
            DrawCubeWires(cubePosition, 2.0, 2.0, 2.0, Color::RAYWHITE);
            DrawGrid(10, 1.0);

            // NOTE: Internal buffers drawing (3D data)
            ffi::rlglDraw();
            //-----------------------------------------------

            // Draw '2D' elements in the scene (GUI)
            //-----------------------------------------------
            // #define RLGL_CREATE_MATRIX_MANUALLY
            // #if defined(RLGL_CREATE_MATRIX_MANUALLY)
            //             matProj = MatrixOrtho(0.0, screenWidth, screenHeight, 0.0, 0.0, 1.0);
            //             matView = MatrixIdentity();

            //             SetMatrixModelview(matView);    // Set internal modelview matrix (default shader)
            //             SetMatrixProjection(matProj);   // Set internal projection matrix (default shader)

            // #else   // Let rlgl generate and multiply matrix internally

            ffi::rlMatrixMode(ffi::RL_PROJECTION as i32); // Enable internal projection matrix
            ffi::rlLoadIdentity(); // Reset internal projection matrix
            ffi::rlOrtho(0.0, screenWidth as f64, screenHeight as f64, 0.0, 0.0, 1.0); // Recalculate internal projection matrix
            ffi::rlMatrixMode(ffi::RL_MODELVIEW as i32); // Enable internal modelview matrix
            ffi::rlLoadIdentity(); // Reset internal modelview matrix
                                   // #endif
            DrawRectangleV(rvec2(10.0, 10.0), rvec2(780.0, 20.0), Color::DARKGRAY);

            // NOTE: Internal buffers drawing (2D data)
            ffi::rlglDraw();
            //-----------------------------------------------

            //----------------------------------------------------------------------------------
        }

        // De-Initialization
        //--------------------------------------------------------------------------------------
    }
}

// Draw rectangle using rlgl OpenGL 1.1 style coding (translated to OpenGL 3.3 internally)
unsafe fn DrawRectangleV(position: Vector2, size: Vector2, color: Color) {
    ffi::rlBegin(ffi::RL_TRIANGLES as i32);
    ffi::rlColor4ub(color.r, color.g, color.b, color.a);

    ffi::rlVertex2i(position.x as i32, position.y as i32);
    ffi::rlVertex2i(position.x as i32, (position.y + size.y) as i32);
    ffi::rlVertex2i((position.x + size.x) as i32, (position.y + size.y) as i32);

    ffi::rlVertex2i(position.x as i32, position.y as i32);
    ffi::rlVertex2i((position.x + size.x) as i32, (position.y + size.y) as i32);
    ffi::rlVertex2i((position.x + size.x) as i32, position.y as i32);
    ffi::rlEnd();
}

// Draw a grid centered at (0, 0, 0)
unsafe fn DrawGrid(slices: i32, spacing: f32) {
    let halfSlices = slices / 2;

    ffi::rlBegin(ffi::RL_LINES as i32);
    for i in -halfSlices..=halfSlices {
        if i == 0 {
            ffi::rlColor3f(0.5, 0.5, 0.5);
            ffi::rlColor3f(0.5, 0.5, 0.5);
            ffi::rlColor3f(0.5, 0.5, 0.5);
            ffi::rlColor3f(0.5, 0.5, 0.5);
        } else {
            ffi::rlColor3f(0.75, 0.75, 0.75);
            ffi::rlColor3f(0.75, 0.75, 0.75);
            ffi::rlColor3f(0.75, 0.75, 0.75);
            ffi::rlColor3f(0.75, 0.75, 0.75);
        }

        ffi::rlVertex3f(i as f32 * spacing, 0.0, -halfSlices as f32 * spacing);
        ffi::rlVertex3f(i as f32 * spacing, 0.0, halfSlices as f32 * spacing);

        ffi::rlVertex3f(-halfSlices as f32 * spacing, 0.0, i as f32 * spacing);
        ffi::rlVertex3f(halfSlices as f32 * spacing, 0.0, i as f32 * spacing);
    }
    ffi::rlEnd();
}

// Draw cube
// NOTE: Cube position is the center position
unsafe fn DrawCube(position: Vector3, width: f32, height: f32, length: f32, color: Color) {
    let x = 0.0;
    let y = 0.0;
    let z = 0.0;

    ffi::rlPushMatrix();

    // NOTE: Be careful! Function order matters (rotate -> scale -> translate)
    ffi::rlTranslatef(position.x, position.y, position.z);
    //rlScalef(2.0f, 2.0f, 2.0f);
    //rlRotatef(45, 0, 1, 0);

    ffi::rlBegin(ffi::RL_TRIANGLES as i32);
    ffi::rlColor4ub(color.r, color.g, color.b, color.a);

    // Front Face -----------------------------------------------------
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Left
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Right
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z + length / 2.0); // Top Left

    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z + length / 2.0); // Top Right
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z + length / 2.0); // Top Left
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Right

    // Back Face ------------------------------------------------------
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Left
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z - length / 2.0); // Top Left
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Right

    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z - length / 2.0); // Top Right
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Right
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z - length / 2.0); // Top Left

    // Top Face -------------------------------------------------------
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z - length / 2.0); // Top Left
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z + length / 2.0); // Bottom Left
    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z + length / 2.0); // Bottom Right

    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z - length / 2.0); // Top Right
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z - length / 2.0); // Top Left
    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z + length / 2.0); // Bottom Right

    // Bottom Face ----------------------------------------------------
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z - length / 2.0); // Top Left
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Right
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Left

    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z - length / 2.0); // Top Right
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Right
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z - length / 2.0); // Top Left

    // Right face -----------------------------------------------------
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Right
    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z - length / 2.0); // Top Right
    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z + length / 2.0); // Top Left

    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Left
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Right
    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z + length / 2.0); // Top Left

    // Left Face ------------------------------------------------------
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Right
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z + length / 2.0); // Top Left
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z - length / 2.0); // Top Right

    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Left
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z + length / 2.0); // Top Left
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Right
    ffi::rlEnd();
    ffi::rlPopMatrix();
}

// Draw cube wires
unsafe fn DrawCubeWires(position: Vector3, width: f32, height: f32, length: f32, color: Color) {
    let x = 0.0;
    let y = 0.0;
    let z = 0.0;

    ffi::rlPushMatrix();

    ffi::rlTranslatef(position.x, position.y, position.z);
    //rlRotatef(45, 0, 1, 0);

    ffi::rlBegin(ffi::RL_LINES as i32);
    ffi::rlColor4ub(color.r, color.g, color.b, color.a);

    // Front Face -----------------------------------------------------
    // Bottom Line
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Left
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Right

    // Left Line
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Right
    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z + length / 2.0); // Top Right

    // Top Line
    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z + length / 2.0); // Top Right
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z + length / 2.0); // Top Left

    // Right Line
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z + length / 2.0); // Top Left
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Left

    // Back Face ------------------------------------------------------
    // Bottom Line
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Left
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Right

    // Left Line
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Right
    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z - length / 2.0); // Top Right

    // Top Line
    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z - length / 2.0); // Top Right
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z - length / 2.0); // Top Left

    // Right Line
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z - length / 2.0); // Top Left
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Left

    // Top Face -------------------------------------------------------
    // Left Line
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z + length / 2.0); // Top Left Front
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z - length / 2.0); // Top Left Back

    // Right Line
    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z + length / 2.0); // Top Right Front
    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z - length / 2.0); // Top Right Back

    // Bottom Face  ---------------------------------------------------
    // Left Line
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z + length / 2.0); // Top Left Front
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z - length / 2.0); // Top Left Back

    // Right Line
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z + length / 2.0); // Top Right Front
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z - length / 2.0); // Top Right Back
    ffi::rlEnd();
    ffi::rlPopMatrix();
}
