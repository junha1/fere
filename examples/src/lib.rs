use fere::prelude::*;
use rand::prelude::*;
use rops::*;

pub fn draw_grid(
    color: IVec4,
    xcolor: IVec4,
    ycolor: IVec4,
    count: usize,
    interval: f32,
    width: f32,
    z_offset: f32,
) -> RenderOp {
    let count = count as i32;
    let z = z_offset;
    let max = interval * count as f32;
    let mut ops_list = Vec::new();
    for c in -count..=count {
        let i = interval * c as f32;
        if c == 0 {
            ops_list.push(rops::Line {
                pos1: Vec3::new(0.0, 0.0, z),
                pos2: Vec3::new(max, 0.0, z),
                color: xcolor,
                width,
            });
            ops_list.push(rops::Line {
                pos1: Vec3::new(0.0, 0.0, z),
                pos2: Vec3::new(0.0, max, z),
                color: ycolor,
                width,
            });
            ops_list.push(rops::Line {
                pos1: Vec3::new(0.0, -max, z),
                pos2: Vec3::new(0.0, 0.0, z),
                color,
                width,
            });
            ops_list.push(rops::Line {
                pos1: Vec3::new(-max, 0.0, z),
                pos2: Vec3::new(0.0, 0.0, z),
                color,
                width,
            });
        } else {
            ops_list.push(rops::Line {
                pos1: Vec3::new(i, -max, z),
                pos2: Vec3::new(i, max, z),
                color,
                width,
            });
            ops_list.push(rops::Line {
                pos1: Vec3::new(-max, i, z),
                pos2: Vec3::new(max, i, z),
                color,
                width,
            });
        }
    }
    RenderOp::Multiple(ops_list.into_iter().map(RenderOp::DrawLine).collect())
}

pub fn gen_color() -> IVec3 {
    let mut color = Vec3::new(0.0, 0.0, 0.0);
    let mut rng = thread_rng();

    for i in 0..3 {
        color[i] = rng.gen_range(0.01..1.0);
    }
    let max = color
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let color = color * (1.0 / max) * 255.0;
    IVec3::new(color.x as i32, color.y as i32, color.z as i32)
}
