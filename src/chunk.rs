use bevy::{render::pipeline::PrimitiveTopology,
           render::mesh::Mesh,
           render::mesh::Indices,
};

use noise::{NoiseFn, Fbm};

const CHUNKSIZE: i8 = 16;

pub fn chunk_coords(x: i16, y: i16, z: i16) -> (i16, i16, i16) {
    (x / CHUNKSIZE as i16, y / CHUNKSIZE as i16, z / CHUNKSIZE as i16)
}

pub struct Chunk {
    pub is_empty: bool,
    pub cxyz: (i16, i16, i16),
    pub blocks: [[[i8; CHUNKSIZE as usize]; CHUNKSIZE as usize]; CHUNKSIZE as usize],
}

impl Chunk {
    pub fn generate(noisem: Fbm, coeff: &f64, cx: f64, cy: f64, cz: f64) -> Self {
        let mut is_empty = true;
        let mut blocks = [[[0i8; CHUNKSIZE as usize]; CHUNKSIZE as usize]; CHUNKSIZE as usize];
        for z in 0..CHUNKSIZE {
            for y in 0..CHUNKSIZE {
                for x in 0..CHUNKSIZE {
                    let v = noisem.get([
                        (x as f64 + CHUNKSIZE as f64 * &cx) / coeff,
                        (y as f64 + CHUNKSIZE as f64 * &cy) / coeff,
                        (z as f64 + CHUNKSIZE as f64 * &cz) / coeff,
                    ]);
                    let ay = &cy * CHUNKSIZE as f64 + y as f64;
                    let b = ((ay - 32.0) * 0.0025).clamp(-0.25, 1.0);

                    if v - b > 0.0 {
                        blocks[x as usize][y as usize][z as usize] = 1;
                        is_empty = false;
                    }
                }
            }
        }
        Self {
            is_empty,
            cxyz: (cx as i16, cy as i16, cz as i16),
            blocks,
        }
    }

    pub fn get_block(&self, x: i8, y: i8, z: i8) -> i8 {
        let chunkrange = 0..CHUNKSIZE;
        return if chunkrange.contains(&x) && chunkrange.contains(&y) && chunkrange.contains(&z) {
            self.blocks[x as usize][y as usize][z as usize] as i8
        } else {
            0
        }
    }

    pub fn make_mesh(&self) -> Mesh {
        let mut vertices = vec![];
        let mut indices = vec![];
        let mut ni = 0;
        for z in 0..CHUNKSIZE {
            for y in 0..CHUNKSIZE {
                for x in 0..CHUNKSIZE {
                    if self.get_block(x, y, z) == 1 {
                        if self.get_block(x, y + 1, z) == 0 {
                            // Top
                            let mut top = vec![
                                ([x as f32 + 0.0, y as f32 + 1.0, z as f32 + 0.0], [0.0, 1.0, 0.0], [0.0, 0.0]),
                                ([x as f32 + 0.0, y as f32 + 1.0, z as f32 + 1.0], [0.0, 1.0, 0.0], [0.0, 1.0]),
                                ([x as f32 + 1.0, y as f32 + 1.0, z as f32 + 0.0], [0.0, 1.0, 0.0], [1.0, 0.0]),
                                ([x as f32 + 1.0, y as f32 + 1.0, z as f32 + 1.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
                            ];
                            vertices.append(&mut top);
                            ni += 4;
                            let mut ix = vec![&ni - 4, &ni - 3, &ni - 2, &ni - 1, &ni - 2, &ni - 3];
                            //let mut ix = vec![&ni-4, &ni-2, &ni-3, &ni-3, &ni-2, &ni-1];
                            indices.append(&mut ix);
                        }
                        if self.get_block(x, y - 1, z) == 0 {
                            // Bottom
                            let mut bottom = vec![
                                ([x as f32 + 0.0, y as f32 + 0.0, z as f32 + 0.0], [0.0, -1.0, 0.0], [1.0, 0.0]),
                                ([x as f32 + 0.0, y as f32 + 0.0, z as f32 + 1.0], [0.0, -1.0, 0.0], [0.0, 1.0]),
                                ([x as f32 + 1.0, y as f32 + 0.0, z as f32 + 0.0], [0.0, -1.0, 0.0], [1.0, 0.0]),
                                ([x as f32 + 1.0, y as f32 + 0.0, z as f32 + 1.0], [0.0, -1.0, 0.0], [1.0, 1.0]),
                            ];
                            vertices.append(&mut bottom);
                            ni += 4;
                            //let mut ix = vec![&ni-4, &ni-3, &ni-2, &ni-1, &ni-2, &ni-3];
                            let mut ix = vec![&ni - 4, &ni - 2, &ni - 3, &ni - 3, &ni - 2, &ni - 1];
                            indices.append(&mut ix);
                        }
                        if self.get_block(x, y, z - 1) == 0 {
                            // North
                            let mut north = vec![
                                ([x as f32 + 0.0, y as f32 + 0.0, z as f32 + 0.0], [0.0, 0.0, -1.0], [0.0, 0.0]),
                                ([x as f32 + 0.0, y as f32 + 1.0, z as f32 + 0.0], [0.0, 0.0, -1.0], [0.0, 1.0]),
                                ([x as f32 + 1.0, y as f32 + 0.0, z as f32 + 0.0], [0.0, 0.0, -1.0], [1.0, 0.0]),
                                ([x as f32 + 1.0, y as f32 + 1.0, z as f32 + 0.0], [0.0, 0.0, -1.0], [1.0, 1.0]),
                            ];
                            vertices.append(&mut north);
                            ni += 4;
                            let mut ix = vec![&ni - 4, &ni - 3, &ni - 2, &ni - 1, &ni - 2, &ni - 3];
                            //let mut ix = vec![&ni-4, &ni-2, &ni-3, &ni-3, &ni-2, &ni-1];
                            indices.append(&mut ix);
                        }
                        if self.get_block(x, y, z + 1) == 0 {
                            // South
                            let mut south = vec![
                                ([x as f32 + 0.0, y as f32 + 0.0, z as f32 + 1.0], [0.0, 0.0, 1.0], [0.0, 0.0]),
                                ([x as f32 + 0.0, y as f32 + 1.0, z as f32 + 1.0], [0.0, 0.0, 1.0], [0.0, 1.0]),
                                ([x as f32 + 1.0, y as f32 + 0.0, z as f32 + 1.0], [0.0, 0.0, 1.0], [1.0, 0.0]),
                                ([x as f32 + 1.0, y as f32 + 1.0, z as f32 + 1.0], [0.0, 0.0, 1.0], [1.0, 1.0]),
                            ];
                            vertices.append(&mut south);
                            ni += 4;
                            //let mut ix = vec![&ni-4, &ni-3, &ni-2, &ni-1, &ni-2, &ni-3];
                            let mut ix = vec![&ni - 4, &ni - 2, &ni - 3, &ni - 3, &ni - 2, &ni - 1];
                            indices.append(&mut ix);
                        }
                        if self.get_block(x + 1, y, z) == 0 {
                            // East
                            let mut east = vec![
                                ([x as f32 + 1.0, y as f32 + 0.0, z as f32 + 0.0], [1.0, 0.0, 0.0], [0.0, 0.0]),
                                ([x as f32 + 1.0, y as f32 + 1.0, z as f32 + 0.0], [1.0, 0.0, 0.0], [0.0, 1.0]),
                                ([x as f32 + 1.0, y as f32 + 0.0, z as f32 + 1.0], [1.0, 0.0, 0.0], [1.0, 0.0]),
                                ([x as f32 + 1.0, y as f32 + 1.0, z as f32 + 1.0], [1.0, 0.0, 0.0], [1.0, 1.0]),
                            ];
                            vertices.append(&mut east);
                            ni += 4;
                            let mut ix = vec![&ni - 4, &ni - 3, &ni - 2, &ni - 1, &ni - 2, &ni - 3];
                            //let mut ix = vec![&ni-4, &ni-2, &ni-3, &ni-3, &ni-2, &ni-1];
                            indices.append(&mut ix);
                        }
                        if self.get_block(x - 1, y, z) == 0 {
                            // West
                            let mut west = vec![
                                ([x as f32 + 0.0, y as f32 + 0.0, z as f32 + 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0]),
                                ([x as f32 + 0.0, y as f32 + 1.0, z as f32 + 0.0], [-1.0, 0.0, 0.0], [0.0, 1.0]),
                                ([x as f32 + 0.0, y as f32 + 0.0, z as f32 + 1.0], [-1.0, 0.0, 0.0], [1.0, 0.0]),
                                ([x as f32 + 0.0, y as f32 + 1.0, z as f32 + 1.0], [-1.0, 0.0, 0.0], [1.0, 1.0]),
                            ];
                            vertices.append(&mut west);
                            ni += 4;
                            //let mut ix = vec![&ni-4, &ni-3, &ni-2, &ni-1, &ni-2, &ni-3];
                            let mut ix = vec![&ni - 4, &ni - 2, &ni - 3, &ni - 3, &ni - 2, &ni - 1];
                            indices.append(&mut ix);
                        }
                    }
                }
            }
        }

        let vlen = vertices.len();
        let mut positions = Vec::with_capacity(vlen);
        let mut normals = Vec::with_capacity(vlen);
        let mut uvs = Vec::with_capacity(vlen);

        // println!("{}, {}, {} -- {}", self.cxyz.0, self.cxyz.1, self.cxyz.2, vlen);

        for (position, normal, uv) in vertices.iter() {
            positions.push(*position);
            normals.push(*normal);
            uvs.push(*uv);
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}
