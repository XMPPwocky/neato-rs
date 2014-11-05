//! Wraps the underlying graphics API (OpenGL).
#![feature(globs, macro_rules)]

extern crate gl;

use std::sync::Arc;

mod raiigl;

fn check_glerror() {
    assert_eq!(unsafe { gl::GetError() }, gl::NO_ERROR)
}
pub struct Backend {
    raiigl: raiigl::RAIIGL,
    marker_nosend: std::kinds::marker::NoSend
}

impl Backend {
    pub fn start_frame(&mut self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT)
        }
    }

    pub fn end_frame(&mut self) {
        self.raiigl.update()
    }

   /* pub fn render_mesh(&mut self, mesh: &Mesh) {
        unimplemented!()
    }

    pub fn use_material(&mut self, material: Material) {
        unimplemented!()
        //self.active_material = material;

       // gl::UseProgram(self.active_material.handle);
        
        check_glerror();
    }*/
}
