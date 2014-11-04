//! Wraps the underlying graphics API (OpenGL).
#![feature(globs)]

extern crate gl;

use std::sync::Arc;

mod shader;

fn check_glerror() {
    assert_eq!(gl::GetError(), gl::NO_ERROR)
}
enum Death {
    ShaderDeath(gl::types::GLuint),
    ShaderProgramDeath(gl::types::GLuint)
}
impl Death {
    fn process(self) {
        match self {
            ShaderDeath(handle) => {
                gl::DeleteShader(handle);
            },
            ShaderProgramDeath(handle) => {
                gl::DeleteProgram(handle);
            }
        }
        check_glerror();
    }
}
pub struct Backend {
    // cloned often
    death_sender: Sender<Death>,
    deaths: Receiver<Death>,

    packets: Vec<RenderPacket>
}

pub struct Material {
    shaders: shader::ShaderProgram,

    // TODO: uniforms, etc. must go here eventually
}

pub enum RenderPacket {
    

impl Backend {
    pub fn render(&mut self) {
        loop {
            match self.deaths.try_recv() {
                Ok(death) => death.process(),
                Err(std::comm::Empty) => break,
                Err(std::comm::Disconnected) => unreachable!(),
            }
        }
    }
}
