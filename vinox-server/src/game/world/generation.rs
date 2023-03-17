use bevy::prelude::*;
use noise::{
    BasicMulti, Blend, MultiFractal, NoiseFn, OpenSimplex, RidgedMulti, RotatePoint, Terrace,
};
use vinox_common::world::chunks::storage::{BlockData, RawChunk, CHUNK_SIZE};

// Just some interesting stuff to look at while testing
pub fn add_grass(raw_chunk: &mut RawChunk) {
    for x in 0..=CHUNK_SIZE - 1 {
        for z in 0..=CHUNK_SIZE - 1 {
            for y in 0..=CHUNK_SIZE - 2 {
                if raw_chunk.get_identifier(UVec3::new(x, y + 1, z)) == "vinox:air"
                    && raw_chunk.get_identifier(UVec3::new(x, y, z)) == "vinox:cobblestone"
                {
                    let grass = BlockData::new("vinox".to_string(), "grass".to_string());
                    raw_chunk.set_block(UVec3::new(x, y, z), &grass);
                }
            }
        }
    }
}

pub fn generate_chunk(pos: IVec3, seed: u32) -> RawChunk {
    //TODO: Switch to using ron files to determine biomes and what blocks they should use. For now hardcoding a simplex noise
    let ridged_noise: RidgedMulti<OpenSimplex> = RidgedMulti::new(seed)
        .set_octaves(10)
        .set_frequency(0.00622);
    let d_noise: RidgedMulti<OpenSimplex> =
        RidgedMulti::new(seed).set_octaves(6).set_frequency(0.00781);
    let final_noise = Blend::new(
        Blend::new(
            RotatePoint {
                source: ridged_noise,
                x_angle: 0.212,
                y_angle: 0.321,
                z_angle: -0.1204,
                u_angle: 0.11,
            },
            RotatePoint {
                source: d_noise,
                x_angle: -0.124,
                y_angle: -0.564,
                z_angle: 0.231,
                u_angle: -0.1151,
            },
            BasicMulti::<OpenSimplex>::new(seed)
                .set_octaves(2)
                .set_frequency(0.003415),
        ),
        Terrace::new(
            BasicMulti::<OpenSimplex>::new(seed)
                .set_octaves(3)
                .set_frequency(0.00461),
        )
        .add_control_point(0.0)
        .add_control_point(8.0)
        .add_control_point(16.0)
        .add_control_point(24.0)
        .add_control_point(32.0),
        BasicMulti::<OpenSimplex>::new(seed)
            .set_octaves(1)
            .set_frequency(0.00075),
    );

    let mut raw_chunk = RawChunk::new();
    for x in 0..=CHUNK_SIZE - 1 {
        for z in 0..=CHUNK_SIZE - 1 {
            for y in 0..=CHUNK_SIZE - 1 {
                let full_x = x as i32 + ((CHUNK_SIZE as i32) * pos.x);
                let full_z = z as i32 + ((CHUNK_SIZE as i32) * pos.z);
                let full_y = y as i32 + ((CHUNK_SIZE as i32) * pos.y);
                let noise_val =
                    final_noise.get([full_x as f64, full_y as f64, full_z as f64]) * 45.152;
                if full_y as f64 <= noise_val {
                    raw_chunk.set_block(
                        UVec3::new(x, y, z),
                        &BlockData::new("vinox".to_string(), "cobblestone".to_string()),
                    );
                } else {
                    raw_chunk.set_block(
                        UVec3::new(x, y, z),
                        &BlockData::new("vinox".to_string(), "air".to_string()),
                    );
                }
            }
        }
    }
    add_grass(&mut raw_chunk);
    raw_chunk
}
