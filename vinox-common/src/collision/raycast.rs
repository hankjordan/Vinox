use bevy::prelude::*;

use crate::world::chunks::{
    ecs::{ChunkComp, CurrentChunks},
    positions::world_to_voxel,
    storage::{BlockTable, Chunk, RawChunk, VoxelVisibility},
};
// Takes in absolute world positions returns a chunk pos and a voxel pos for whatever face it hits and a normal
pub fn raycast_world(
    origin: Vec3,
    direction: Vec3,
    radius: f32,
    chunks: &Query<&mut ChunkComp>,
    current_chunks: &CurrentChunks,
    block_table: &BlockTable,
) -> Option<(IVec3, UVec3, Vec3)> {
    let mut origin = origin.floor();
    let step = direction.signum();

    let mut tmax = Vec3::new(
        intbound(origin.x, direction.x),
        intbound(origin.y, direction.y),
        intbound(origin.z, direction.z),
    );

    let tdelta = step / direction;

    let mut face = Vec3::ZERO;

    if direction == Vec3::ZERO {
        return None;
    }

    let radius = radius
        / (direction.x * direction.x + direction.y * direction.y + direction.z * direction.z)
            .sqrt();

    loop {
        let (chunk_pos, voxel_pos) = world_to_voxel(origin);
        if let Some(chunk_entity) = current_chunks.get_entity(chunk_pos) {
            if let Ok(chunk) = chunks.get(chunk_entity) {
                if chunk
                    .chunk_data
                    .get_data(
                        RawChunk::linearize(UVec3::new(voxel_pos.x, voxel_pos.y, voxel_pos.z)),
                        block_table,
                    )
                    .visibility
                    .unwrap()
                    != VoxelVisibility::Empty
                {
                    return Some((chunk_pos, voxel_pos, face));
                }
            }
        }

        if tmax.x < tmax.y {
            if tmax.x < tmax.z {
                if tmax.x > radius {
                    break;
                }
                origin.x += step.x;
                tmax.x += tdelta.x;
                face.x = -step.x;
                face.y = 0.0;
                face.z = 0.0;
            } else {
                if tmax.z > radius {
                    break;
                }
                origin.z += step.z;
                tmax.z += tdelta.z;
                face.x = 0.0;
                face.y = 0.0;
                face.z = -step.z;
            }
        } else {
            if tmax.y < tmax.z {
                if tmax.y > radius {
                    break;
                }
                origin.y += step.y;
                tmax.y += tdelta.y;
                face.x = 0.0;
                face.y = -step.y;
                face.z = 0.0;
            } else {
                if tmax.z > radius {
                    break;
                }
                origin.z += step.z;
                tmax.z += tdelta.z;
                face.x = 0.0;
                face.y = 0.0;
                face.z = -step.z;
            }
        }
    }
    None
}

fn intbound(s: f32, ds: f32) -> f32 {
    // Some kind of edge case, see:
    // http://gamedev.stackexchange.com/questions/47362/cast-ray-to-select-block-in-voxel-game#comment160436_49423
    let is_int = s == s.round();
    if ds < 0.0 && is_int {
        return 0.0;
    }

    if ds > 0.0 {
        if s.ceil() == 0.0 {
            return 1.0 - s;
        } else {
            s.ceil() - s
        }
    } else {
        s - (s.floor() / ds.abs())
    }
}
