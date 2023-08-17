use std::{path::PathBuf, fs::File, io::Read};

use glium::{Program, Display};

pub fn load(display: &Display, fragment: PathBuf, vertex: PathBuf) -> Program {
    let mut v_file = File::open(vertex).unwrap();
    let mut f_file = File::open(fragment).unwrap();

    let mut v_src = String::new();
    let mut f_src = String::new();
    
    v_file.read_to_string(&mut v_src).unwrap();
    f_file.read_to_string(&mut f_src).unwrap();

    Program::from_source(display, &v_src, &f_src, None).expect("Failed to compile shaders")
}
