extern crate gl;
extern crate rgl;
extern crate glfw;

use glfw::{Action, Context, Key};
use gl::types::*;
use std::ptr;
use std::str;

static VERTEX_DATA: [GLfloat; 6] = [
    0.0, 0.5,
    0.5, -0.5,
    -0.5, -0.5,
];

fn compile_shader(src: &str, shader_type: rgl::ShaderType) -> rgl::Shader {
    let shader = rgl::create_shader(shader_type);
    rgl::shader_source(shader, &src);
    rgl::compile_shader(shader);

    let mut status = gl::FALSE as GLint;
    rgl::get_shader_iv(shader, rgl::ShaderInfoParam::CompileStatus, &mut status);

    if status != (gl::TRUE as GLint) {
        let mut len = 0;
        rgl::get_shader_iv(shader, rgl::ShaderInfoParam::InfoLogLength, &mut len);

        let mut buf = Vec::with_capacity(len as usize);

        unsafe {
            buf.set_len((len as usize) - 1); // Skip null character

            gl::GetShaderInfoLog(
                shader.into(),
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
        }

        panic!("{}", str::from_utf8(&buf).ok().expect("ShaderInfo not valid utf8"));
    }
    shader
}

fn link_program(vs: rgl::Shader, fs: rgl::Shader) -> rgl::Program {
    let program = rgl::create_program();
    rgl::attach_shader(program, vs);
    rgl::attach_shader(program, fs);
    rgl::link_program(program);

    unsafe {
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program.into(), gl::LINK_STATUS, &mut status);

        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetProgramiv(program.into(), gl::INFO_LOG_LENGTH, &mut len);

            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // Skip null character

            gl::GetProgramInfoLog(
                program.into(),
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!("{}", str::from_utf8(&buf).ok().expect("ShaderInfo not valid utf8"));
        }
    }

    program
}

fn main() {
    // Initialize glfw and create window
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));

    let (mut window, events) = glfw.create_window(
        300, 300,
        "Hello, this is window",
        glfw::WindowMode::Windowed
    ).expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    // Load gl function pointers
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    let (window_width, window_height) = window.get_size();
    unsafe {
        gl::Viewport(0, 0, window_width, window_height);
    }

    let vs = compile_shader(include_str!("shader.vert"), rgl::ShaderType::Vertex);
    let fs = compile_shader(include_str!("shader.frag"), rgl::ShaderType::Fragment);
    let program = link_program(vs, fs);

    // Upload triangle vertex data
    let vao = rgl::gen_vertex_array();
    rgl::bind_vertex_array(vao);

    let vbo = rgl::gen_buffer();
    rgl::bind_buffer(rgl::Target::ArrayBuffer, vbo);
    rgl::buffer_data(rgl::Target::ArrayBuffer, &VERTEX_DATA, rgl::Usage::StaticDraw);

    rgl::use_program(program);

    let pos_attr = rgl::get_attrib_location(program, "position");

    rgl::enable_vertex_attrib_array(pos_attr as GLuint);
    rgl::vertex_attrib_pointer(pos_attr as GLuint, 2, rgl::Type::Float, false, 0);

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        rgl::clear_color(0.3, 0.3, 0.3, 1.0);
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };

        rgl::draw_arrays(rgl::Primitive::Triangles, 0, 3);

        window.swap_buffers();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        glfw::WindowEvent::Size(w, h) => {
            unsafe { gl::Viewport(0, 0, w, h) }
        }
        _ => {}
    }
}
