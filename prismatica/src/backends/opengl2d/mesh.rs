use std::{os::raw::c_void, path::Path};

#[repr(packed)]
pub struct OpenGLVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}

pub struct Texture {
    pub id: u32
}

impl Texture {
    pub fn new() -> Self {
        Texture {
            id: 0
        }
    }

    pub fn from_file(path: &Path) -> Self {
        let image = image::ImageReader::open(path).unwrap().decode().unwrap().flipv();

        let mut texture: u32 = 1;
        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB.try_into().unwrap(), image.width() as i32, image.height() as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, image.to_rgb8().as_ptr() as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        Texture { id: texture }
    }
}

pub struct Mesh {
    pub vertices: Vec<OpenGLVertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,
    pub vao: u32,
    pub vbo: u32,
    pub ebo: u32,
}

impl Mesh {
    pub fn new(vertices: Vec<OpenGLVertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<OpenGLVertex>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<OpenGLVertex>() as i32,
                std::ptr::null());
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<OpenGLVertex>() as i32,
                (std::mem::size_of::<glm::Vec3>()) as *const _);
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<OpenGLVertex>() as i32,
                (2 * std::mem::size_of::<glm::Vec3>()) as *const _);

            gl::BindVertexArray(0);
        }

        Mesh {
            vertices,
            indices,
            textures,
            vao,
            vbo,
            ebo,
        }
    }
}