use advtools::input;
use advtools::vecs::{cgmath::dot, f64::Vec3};
use advtools::prelude::Itertools;
use std::ops::Range;

const RX: &str = r"(\d+), (\d+), (\d+) @ (-?\d+), (-?\d+), (-?\d+)";

const TEST_AREA: Range<f64> = (200000000000000.)..(400000000000000.);

fn is_int(x: f64) -> bool {
    x.fract() == 0.0
}

fn line_plane_intersection(line_start: Vec3, line_dir: Vec3, plane_point: Vec3, plane_normal: Vec3) -> Vec3 {
    let line_origin_to_plane_point = line_start - plane_point;
    let dot_product = dot(line_origin_to_plane_point, plane_normal);
    let line_direction_dot_plane_normal = dot(line_dir, plane_normal);
    let t = -dot_product / line_direction_dot_plane_normal;
    line_start + t * line_dir
}

fn main() {
    let mut stones = input::rx_lines::<(Vec3, Vec3)>(RX).collect_vec();

    let mut n = 0;
    for ((p1, v1), (p2, v2)) in stones.iter().tuple_combinations() {
        let m1 = v1.y / v1.x;
        let m2 = v2.y / v2.x;
        let q1 = p1.y - m1 * p1.x;
        let q2 = p2.y - m2 * p2.x;
        if m1 != m2 {
            let cross_x = (q2 - q1) / (m1 - m2);
            let cross_y = m1 * cross_x + q1;
            let t1 = (cross_x - p1.x) / v1.x;
            let t2 = (cross_x - p2.x) / v2.x;
            if t1 >= 0. && t2 >= 0. && TEST_AREA.contains(&cross_x) && TEST_AREA.contains(&cross_y) {
                n += 1;
            }
        }
    }
    advtools::verify("Intersections", n, 31208);

    stones.sort_by(|a, b| a.1.x.partial_cmp(&b.1.x).unwrap());
    let vx_pairs = stones.iter().tuple_windows().filter(|((_, v1), (_, v2))| v1.x == v2.x)
                                                .map(|((p1, v1), (p2, _))| ((p1.x - p2.x).abs(), v1.x))
                                                .collect_vec();
    let mut vx_candidates = (-500..=500).map(|v| v as f64)
                                        .filter(|vxc| vx_pairs.iter().all(|(dx, vx)| vxc != vx && dx % (vxc - vx).abs() == 0.))
                                        .collect_vec();
    if vx_candidates.is_empty() {
        vx_candidates = stones.iter().map(|(_, v)| v.x).collect_vec();
    }

    stones.sort_by(|a, b| a.1.y.partial_cmp(&b.1.y).unwrap());
    let vy_pairs = stones.iter().tuple_windows().filter(|((_, v1), (_, v2))| v1.y == v2.y)
                                                .map(|((p1, v1), (p2, _))| ((p1.y - p2.y).abs(), v1.y))
                                                .collect_vec();
    let vy_candidates = (-500..=500).map(|v| v as f64)
                                    .filter(|vyc| vy_pairs.iter().all(|(dy, vy)| vyc != vy && dy % (*vyc - vy).abs() == 0.))
                                    .collect_vec();

    stones.sort_by(|a, b| a.1.z.partial_cmp(&b.1.z).unwrap());
    let vz_pairs = stones.iter().tuple_windows().filter(|((_, v1), (_, v2))| v1.z == v2.z)
                                                .map(|((p1, v1), (p2, _))| ((p1.z - p2.z).abs(), v1.z))
                                                .collect_vec();
    let vz_candidates = (-500..=500).map(|v| v as f64)
                                    .filter(|vzc| vz_pairs.iter().all(|(dz, vz)| vzc != vz && dz % (*vzc - vz).abs() == 0.))
                                    .collect_vec();

    let (p_1, v_1) = stones[0];
    let (p_2, v_2) = stones[1];
    'outer:
    for &vxc in &vx_candidates {
        for &vyc in &vy_candidates {
            for &vzc in &vz_candidates {
                let v_s = Vec3::new(vxc, vyc, vzc);
                let plane_normal = v_1.cross(v_s);
                let ix = line_plane_intersection(p_2, v_2, p_1, plane_normal);
                if !is_int(ix.x) || !is_int(ix.y) || !is_int(ix.z) {
                    continue;
                }
                let t = ((ix.x - p_2.x) / v_2.x).abs();
                let sum_coords = (ix.x - t * vxc) as i64 +
                    (ix.y - t * vyc) as i64 +
                    (ix.z - t * vzc) as i64;
                advtools::verify("Sum of starting coords", sum_coords, 580043851566574_i64);
                break 'outer;
            }
        }
    }
}
