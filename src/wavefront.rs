use std::{
    fs::File,
    io::{BufRead, BufReader, Cursor, Read},
    path::PathBuf,
};

use crate::vertex::Vertex;

pub fn load(path: PathBuf) -> (Vec<Vertex>, Vec<u32>) {
    let mut file = File::open(path).expect("Could not open .obj file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read .obj file.");
    let mut stream = BufReader::new(Cursor::new(contents));
    let mut v_data = vec![];
    let mut i_data = vec![];
    let mut ni_data = vec![];
    let mut vn_data = vec![];

    let mut current_line = String::new();
    while stream
        .read_line(&mut current_line)
        .expect("Malformed line in obj file")
        != 0
    {
        // Skip empty lines and comments
        if current_line.is_empty()
            || current_line == "\n"
            || current_line.chars().next().unwrap() == '#'
        {
            current_line.clear();
            continue;
        }

        let mut split = current_line.strip_suffix("\n").unwrap().split(' ');
        let command = split.next().unwrap();
        match command {
            "o" => {}
            "v" => {
                let x: f32 = split.next().unwrap().parse().unwrap();
                let y: f32 = split.next().unwrap().parse().unwrap();
                let z: f32 = split.next().unwrap().parse().unwrap();
                v_data.push((x, y, z));
            }
            "f" => {
                // Assume triangulated faces
                for _ in 0..3 {
                    let index_str = split.next().expect("malformed index string");
                    let mut split = index_str.split("/");
                    let vi: u32 = split
                        .next()
                        .expect("Expected vertex index")
                        .parse()
                        .unwrap();
                    let _ti = split.next().expect("Expected vertex texture coord index"); //.parse().unwrap();
                    let ni: u32 = split
                        .next()
                        .expect("Expected vertex normal index")
                        .parse()
                        .unwrap();

                    i_data.push(vi - 1);
                    ni_data.push(ni - 1);
                }
            }
            "vn" => {
                let x: f32 = split.next().unwrap().parse().unwrap();
                let y: f32 = split.next().unwrap().parse().unwrap();
                let z: f32 = split.next().unwrap().parse().unwrap();
                vn_data.push((x, y, z));
            }
            "s" => {}
            _ => {
                panic!("Unrecognized .obj command.")
            }
        }
        current_line.clear();
    }

    // All vertices including doubles on corners
    let mut vert_list: Vec<Vertex> = vec![];

    for (vertices, normals) in i_data.chunks(3).zip(ni_data.chunks(3)) {
        for i in 0..3 {
            let vertex_idx = vertices[i] as usize;
            let vertex = v_data[vertex_idx];

            let normal_idx = normals[i] as usize;
            let normal = vn_data[normal_idx];

            vert_list.push(Vertex{
                position: [vertex.0, vertex.1, vertex.2],
                normal: [normal.0, normal.1, normal.2],
                
            });
        }
    }

    (vert_list, i_data)
}
