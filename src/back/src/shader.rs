use {Death, ShaderDeath, ShaderProgramDeath};
use std::sync::Arc;
use gl::types::*;

pub struct Shader {
    handle: GLuint,
    fate: Sender<Death>
}
impl Drop for Shader {
    fn drop(&mut self) {
        self.fate.send(ShaderDeath(self.handle))
    }
}

pub struct ShaderProgram {
    handle: GLuint,
    fate: Sender<Death>
}
impl Drop for ShaderProgram {
    fn drop(&mut self) {
        self.fate.send(ShaderProgramDeath(self.handle))
    }
}
