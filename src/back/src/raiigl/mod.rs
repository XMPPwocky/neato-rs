use std::kinds::marker::NoSend;
use gl;
use gl::types::*;

unsafe fn get_glresult() -> Result<(), GLenum> {
    match gl::GetError() {
        gl::NO_ERROR => Ok(()),
        e => Err(e)
    }
}

pub struct RAIIGL {
    deaths: Receiver<Death>,
    death_sender: Sender<Death>,
    marker_nosend: NoSend
}

impl RAIIGL {
    pub fn new() -> RAIIGL {
        let (tx, rx) = channel();
        RAIIGL {
            deaths: rx,
            death_sender: tx,
            marker_nosend: NoSend
        }
    }

    pub fn update(&mut self) {
        loop {
            match self.deaths.try_recv() {
                Ok(death) => unsafe { death.process() },
                Err(::std::comm::Disconnected) => unreachable!(),
                Err(::std::comm::Empty) => break
            }
        }
    }

    pub fn create_shader(&self, shader_type: GLenum) -> Result<Shader, GLenum> {
        unsafe {
            let shader = gl::CreateShader(shader_type);
            get_glresult().map(|_| Shader {
                handle: shader,
                fate: self.death_sender.clone()
            })
        }
    }

    pub fn create_program(&mut self) -> Result<ShaderProgram, GLenum> {
        unsafe {
            let program = gl::CreateProgram();
            get_glresult().map(|_| ShaderProgram {
                handle: program,
                fate: self.death_sender.clone()
            })
        }
    }

    pub fn create_buffer(&mut self) -> Result<BufferObject, GLenum> {
        unsafe {
            let mut buffer = 0;
            gl::GenBuffers(1, &mut buffer);
            
            get_glresult().map(|_| BufferObject {
                handle: buffer,
                fate: self.death_sender.clone()
            })
        }
    }
}

enum Death {
    ShaderDeath(GLuint),
    ShaderProgramDeath(GLuint),
    BufferObjectDeath(GLuint)
}
impl Death {
    unsafe fn process(self) {
        match self {
            ShaderDeath(shader) => gl::DeleteShader(shader),
            ShaderProgramDeath(program) => gl::DeleteProgram(program),
            BufferObjectDeath(buffer) => gl::DeleteBuffers(1, &buffer)
        }
    }
}

// split into a trait for users' control over visibility
pub trait AsHandle<T> {
    fn as_handle(&self) -> T;
}
macro_rules! raiigl_object(
    ($objname:ident, $raw:ty, $deathvariant:ident) => (
        pub struct $objname {
            handle: $raw,
            fate: Sender<Death>
        }

        impl Drop for $objname {
            fn drop(&mut self) {
                self.fate.send($deathvariant(self.handle))
            }
        }

        impl AsHandle<$raw> for $objname {
            fn as_handle(&self) -> $raw {
                self.handle
            }
        }
    )
)

raiigl_object!(Shader, GLuint, ShaderDeath)
raiigl_object!(ShaderProgram, GLuint, ShaderProgramDeath)
raiigl_object!(BufferObject, GLuint, BufferObjectDeath)
