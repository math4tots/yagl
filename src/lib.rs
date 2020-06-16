extern crate winit;
extern crate wgpu;
extern crate futures;
extern crate shaderc;
extern crate bytemuck;
extern crate image;
extern crate failure;
extern crate cgmath;
extern crate anyhow;

pub mod graphics;

mod tut;
mod sandbox;

pub use tut::main;
pub use sandbox::s2d_main;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
