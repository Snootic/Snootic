use crate::structs::{Direction, Face};

pub const BOX: f32 = 156.0;
const CENTER: f32 = BOX / 2.0;
const HALF_FRONT: f32 = 53.0;
const DEPTH: f32 = 4.2;
const PATCH_MIN_Z: f32 = 0.22;

pub type Mat3 = [[f32; 3]; 3];

pub const IDENTITY: Mat3 = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

pub fn mul(a: Mat3, b: Mat3) -> Mat3 {
    let mut out = [[0.0f32; 3]; 3];
    for (r, row) in out.iter_mut().enumerate() {
        for (c, cell) in row.iter_mut().enumerate() {
            *cell = (0..3).map(|k| a[r][k] * b[k][c]).sum();
        }
    }
    out
}

pub fn rot_x(rad: f32) -> Mat3 {
    let (s, c) = rad.sin_cos();
    [[1.0, 0.0, 0.0], [0.0, c, -s], [0.0, s, c]]
}

pub fn rot_z(rad: f32) -> Mat3 {
    let (s, c) = rad.sin_cos();
    [[c, -s, 0.0], [s, c, 0.0], [0.0, 0.0, 1.0]]
}

pub fn rot_y(rad: f32) -> Mat3 {
    let (s, c) = rad.sin_cos();
    [[c, 0.0, s], [0.0, 1.0, 0.0], [-s, 0.0, c]]
}

pub fn apply(m: Mat3, v: [f32; 3]) -> [f32; 3] {
    [
        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
        m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
        m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
    ]
}

pub const VERTS: [[f32; 3]; 8] = [
    [-1.0, -1.0, -1.0],
    [1.0, -1.0, -1.0],
    [-1.0, 1.0, -1.0],
    [1.0, 1.0, -1.0],
    [-1.0, -1.0, 1.0],
    [1.0, -1.0, 1.0],
    [-1.0, 1.0, 1.0],
    [1.0, 1.0, 1.0],
];

pub const FACES: [(Face, [usize; 4]); 6] = [
    (Face::Front, [4, 5, 7, 6]),
    (Face::Right, [1, 5, 7, 3]),
    (Face::Back, [0, 1, 3, 2]),
    (Face::Left, [0, 4, 6, 2]),
    (Face::Top, [3, 7, 6, 2]),
    (Face::Bottom, [1, 5, 4, 0]),
];

pub const EDGES: [(usize, usize, Face, Face); 12] = [
    // along x
    (0, 1, Face::Bottom, Face::Back),
    (2, 3, Face::Top, Face::Back),
    (4, 5, Face::Bottom, Face::Front),
    (6, 7, Face::Top, Face::Front),
    // along y
    (0, 2, Face::Left, Face::Back),
    (1, 3, Face::Right, Face::Back),
    (4, 6, Face::Left, Face::Front),
    (5, 7, Face::Right, Face::Front),
    // along z
    (0, 4, Face::Left, Face::Bottom),
    (1, 5, Face::Right, Face::Bottom),
    (2, 6, Face::Left, Face::Top),
    (3, 7, Face::Right, Face::Top),
];

pub fn project(p: [f32; 3]) -> (f32, f32) {
    let s = HALF_FRONT * (DEPTH - 1.0) / (DEPTH - p[2]);
    (CENTER + p[0] * s, CENTER - p[1] * s)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Edge {
    pub from: (f32, f32),
    pub to: (f32, f32),
    pub hidden: bool,
    pub active: bool,
    pub target: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Patch {
    pub face: Face,
    pub cx: f32,
    pub cy: f32,
    pub active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cube {
    pub m: Mat3,
}

impl Cube {
    pub const fn new(m: Mat3) -> Cube {
        Cube { m }
    }

    pub fn facing(face: Face) -> Cube {
        Cube::new(face_matrix(face, IDENTITY))
    }

    pub fn normals(&self) -> [(Face, [f32; 3]); 6] {
        Face::ALL.map(|face| (face, apply(self.m, face.normal())))
    }

    pub fn is_front(&self, face: Face) -> bool {
        self.normal_z(face) > 0.0
    }

    fn normal_z(&self, face: Face) -> f32 {
        apply(self.m, face.normal())[2]
    }

    pub fn active(&self) -> Face {
        let mut best = Face::Front;
        let mut best_z = f32::NEG_INFINITY;
        for face in Face::ALL {
            let z = self.normal_z(face);
            if z > best_z {
                best_z = z;
                best = face;
            }
        }
        best
    }

    pub fn neighbour(&self, dir: Direction) -> Face {
        let axis = match dir {
            Direction::Left | Direction::Right => 0,
            Direction::Up | Direction::Down => 1,
        };
        let sign = match dir {
            Direction::Left | Direction::Down => 1.0,
            Direction::Right | Direction::Up => -1.0,
        };
        let active = self.active();
        let mut best = Face::Front;
        let mut best_d = f32::NEG_INFINITY;
        for (face, n) in self.normals() {
            if face == active {
                continue;
            }
            let d = n[axis] * sign;
            if d > best_d {
                best_d = d;
                best = face;
            }
        }
        best
    }

    pub fn neighbours(&self) -> [(Direction, Face); 4] {
        [
            (Direction::Left, self.neighbour(Direction::Left)),
            (Direction::Right, self.neighbour(Direction::Right)),
            (Direction::Up, self.neighbour(Direction::Up)),
            (Direction::Down, self.neighbour(Direction::Down)),
        ]
    }

    pub fn edges_toward(&self, target: Option<Face>) -> [Edge; 12] {
        let active = self.active();
        let vertices = VERTS.map(|v| project(apply(self.m, v)));
        EDGES.map(|(a, b, f1, f2)| Edge {
            from: vertices[a],
            to: vertices[b],
            hidden: !self.is_front(f1) && !self.is_front(f2),
            active: f1 == active || f2 == active,
            target: target == Some(f1) || target == Some(f2),
        })
    }

    pub fn edges(&self) -> [Edge; 12] {
        self.edges_toward(None)
    }

    pub fn polygon(&self, face: Face) -> Vec<(f32, f32)> {
        FACES
            .iter()
            .find(|(f, _)| *f == face)
            .map(|(_, vs)| {
                vs.iter()
                    .map(|v| project(apply(self.m, VERTS[*v])))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn clip_path(&self, face: Face) -> String {
        let pts: Vec<String> = self
            .polygon(face)
            .iter()
            .map(|(x, y)| format!("{x:.2}px {y:.2}px"))
            .collect();
        format!("polygon({})", pts.join(","))
    }

    pub fn patches(&self) -> Vec<Patch> {
        let active = self.active();
        let mut out: Vec<Patch> = Face::ALL
            .iter()
            .filter(|f| self.normal_z(**f) > PATCH_MIN_Z)
            .map(|f| {
                let poly = self.polygon(*f);
                let n = poly.len() as f32;
                let (sx, sy) = poly
                    .iter()
                    .fold((0.0, 0.0), |(ax, ay), (x, y)| (ax + x, ay + y));
                Patch {
                    face: *f,
                    cx: sx / n,
                    cy: sy / n,
                    active: *f == active,
                }
            })
            .collect();
        out.sort_by(|a, b| {
            self.normal_z(b.face)
                .partial_cmp(&self.normal_z(a.face))
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        out
    }

    pub fn section(&self) -> String {
        let poly = self.polygon(self.active());
        if poly.is_empty() {
            return String::new();
        }
        let n = poly.len() as f32;
        let (cx, cy) = poly
            .iter()
            .fold((0.0, 0.0), |(ax, ay), (x, y)| (ax + x, ay + y));
        let (cx, cy) = (cx / n, cy / n);
        let k = 0.38;
        let pts: Vec<String> = poly
            .iter()
            .map(|(x, y)| {
                let px = cx + (x - cx) * k;
                let py = cy + (y - cy) * k;
                format!("{px:.2} {py:.2}")
            })
            .collect();
        format!("M{}Z", pts.join("L"))
    }

    pub fn triad(&self) -> [Triad; 3] {
        let names = ["cube.triad.right", "cube.triad.top", "cube.triad.front"];
        let axes = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        std::array::from_fn(|i| {
            let v = apply(self.m, axes[i]);
            Triad {
                label: names[i],
                x: v[0],
                y: -v[1],
                collapsed: v[2].abs() > 0.85,
                toward: v[2] > 0.85,
            }
        })
    }

    pub fn verify(&self) -> Vec<&'static str> {
        let mut problems = Vec::new();
        let active = self.active();

        let z_active = self.normal_z(active);
        if z_active < -1e-4 {
            problems.push("the active face is turned away from the reader");
        }
        for face in Face::ALL {
            if self.normal_z(face) > z_active + 1e-4 {
                problems.push("a face is more face-on than the active one");
                break;
            }
        }

        for (i, (_, _, f1, f2)) in EDGES.iter().enumerate() {
            if (*f1 == active || *f2 == active) && self.edges()[i].hidden {
                problems.push("an edge of the active face is drawn hidden");
                break;
            }
        }

        for v in VERTS {
            let (x, y) = project(apply(self.m, v));
            if !(0.0..=BOX).contains(&x) || !(0.0..=BOX).contains(&y) {
                problems.push("the drawing leaves its box");
                break;
            }
        }

        for dir in [
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
        ] {
            if self.neighbour(dir) == active {
                problems.push("a drag direction reaches the face you are already on");
                break;
            }
        }

        for row in self.m {
            let len = (row[0] * row[0] + row[1] * row[1] + row[2] * row[2]).sqrt();
            if (len - 1.0).abs() > 1e-3 {
                problems.push("the orientation is not orthonormal");
                break;
            }
        }
        let det = self.m[0][0] * (self.m[1][1] * self.m[2][2] - self.m[1][2] * self.m[2][1])
            - self.m[0][1] * (self.m[1][0] * self.m[2][2] - self.m[1][2] * self.m[2][0])
            + self.m[0][2] * (self.m[1][0] * self.m[2][1] - self.m[1][1] * self.m[2][0]);
        if (det - 1.0).abs() > 1e-3 {
            problems.push("the orientation is mirrored");
        }

        problems
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triad {
    pub label: &'static str,
    pub x: f32,
    pub y: f32,
    pub collapsed: bool,
    pub toward: bool,
}

pub fn face_matrix(face: Face, from: Mat3) -> Mat3 {
    let turn = match face {
        Face::Front => IDENTITY,
        Face::Right => rot_y(-std::f32::consts::FRAC_PI_2),
        Face::Back => rot_y(std::f32::consts::PI),
        Face::Left => rot_y(std::f32::consts::FRAC_PI_2),
        Face::Top => rot_x(std::f32::consts::FRAC_PI_2),
        Face::Bottom => rot_x(-std::f32::consts::FRAC_PI_2),
    };
    mul(turn, from)
}

pub fn canonical(face: Face, roll: u8) -> Mat3 {
    let r = (roll % 4) as f32 * std::f32::consts::FRAC_PI_2;
    mul(rot_z(r), face_matrix(face, IDENTITY))
}

pub fn snap_to(from: Mat3, face: Face) -> Mat3 {
    let q = to_quat(from);
    let mut best = canonical(face, 0);
    let mut best_dot = f32::NEG_INFINITY;
    for roll in 0..4 {
        let candidate = canonical(face, roll);
        let cq = to_quat(candidate);
        let dot: f32 = (0..4).map(|i| q[i] * cq[i]).sum::<f32>().abs();
        if dot > best_dot {
            best_dot = dot;
            best = candidate;
        }
    }
    best
}

pub fn is_canonical(m: Mat3) -> bool {
    let cube = Cube::new(m);
    let poly = cube.polygon(cube.active());
    if poly.len() != 4 {
        return false;
    }
    for i in 0..4 {
        let (x1, y1) = poly[i];
        let (x2, y2) = poly[(i + 1) % 4];
        let (dx, dy) = (x2 - x1, y2 - y1);
        if dx.abs() > 1e-3 && dy.abs() > 1e-3 {
            return false;
        }
    }
    true
}

pub fn to_quat(m: Mat3) -> [f32; 4] {
    let trace = m[0][0] + m[1][1] + m[2][2];
    let q = if trace > 0.0 {
        let s = (trace + 1.0).sqrt() * 2.0;
        [
            (m[2][1] - m[1][2]) / s,
            (m[0][2] - m[2][0]) / s,
            (m[1][0] - m[0][1]) / s,
            0.25 * s,
        ]
    } else if m[0][0] > m[1][1] && m[0][0] > m[2][2] {
        let s = (1.0 + m[0][0] - m[1][1] - m[2][2]).sqrt() * 2.0;
        [
            0.25 * s,
            (m[0][1] + m[1][0]) / s,
            (m[0][2] + m[2][0]) / s,
            (m[2][1] - m[1][2]) / s,
        ]
    } else if m[1][1] > m[2][2] {
        let s = (1.0 + m[1][1] - m[0][0] - m[2][2]).sqrt() * 2.0;
        [
            (m[0][1] + m[1][0]) / s,
            0.25 * s,
            (m[1][2] + m[2][1]) / s,
            (m[0][2] - m[2][0]) / s,
        ]
    } else {
        let s = (1.0 + m[2][2] - m[0][0] - m[1][1]).sqrt() * 2.0;
        [
            (m[0][2] + m[2][0]) / s,
            (m[1][2] + m[2][1]) / s,
            0.25 * s,
            (m[1][0] - m[0][1]) / s,
        ]
    };
    normalize_quat(q)
}

pub fn from_quat(q: [f32; 4]) -> Mat3 {
    let [x, y, z, w] = normalize_quat(q);
    [
        [
            1.0 - 2.0 * (y * y + z * z),
            2.0 * (x * y - z * w),
            2.0 * (x * z + y * w),
        ],
        [
            2.0 * (x * y + z * w),
            1.0 - 2.0 * (x * x + z * z),
            2.0 * (y * z - x * w),
        ],
        [
            2.0 * (x * z - y * w),
            2.0 * (y * z + x * w),
            1.0 - 2.0 * (x * x + y * y),
        ],
    ]
}

fn normalize_quat(q: [f32; 4]) -> [f32; 4] {
    let n = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]).sqrt();
    if n < 1e-8 {
        return [0.0, 0.0, 0.0, 1.0];
    }
    [q[0] / n, q[1] / n, q[2] / n, q[3] / n]
}

pub fn slerp(from: Mat3, to: Mat3, t: f32) -> Mat3 {
    let a = to_quat(from);
    let mut b = to_quat(to);
    let mut dot = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3];
    if dot < 0.0 {
        b = [-b[0], -b[1], -b[2], -b[3]];
        dot = -dot;
    }
    if dot > 0.9995 {
        let out = [
            a[0] + (b[0] - a[0]) * t,
            a[1] + (b[1] - a[1]) * t,
            a[2] + (b[2] - a[2]) * t,
            a[3] + (b[3] - a[3]) * t,
        ];
        return from_quat(out);
    }
    let theta = dot.clamp(-1.0, 1.0).acos();
    let sin = theta.sin();
    let wa = ((1.0 - t) * theta).sin() / sin;
    let wb = (t * theta).sin() / sin;
    from_quat([
        a[0] * wa + b[0] * wb,
        a[1] * wa + b[1] * wb,
        a[2] * wa + b[2] * wb,
        a[3] * wa + b[3] * wb,
    ])
}

pub fn same(a: Mat3, b: Mat3) -> bool {
    a.iter()
        .zip(b.iter())
        .all(|(ra, rb)| ra.iter().zip(rb.iter()).all(|(x, y)| (x - y).abs() < 1e-4))
}

pub fn polygon_area(poly: &[(f32, f32)]) -> f32 {
    if poly.len() < 3 {
        return 0.0;
    }
    let mut sum = 0.0;
    for i in 0..poly.len() {
        let (x1, y1) = poly[i];
        let (x2, y2) = poly[(i + 1) % poly.len()];
        sum += x1 * y2 - x2 * y1;
    }
    (sum / 2.0).abs()
}

pub fn line_d(from: (f32, f32), to: (f32, f32)) -> String {
    format!("M{:.2} {:.2}L{:.2} {:.2}", from.0, from.1, to.0, to.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn deg(d: f32) -> f32 {
        d * std::f32::consts::PI / 180.0
    }

    fn orientations() -> Vec<Cube> {
        let mut out = Vec::new();
        for ry in [-180, -135, -90, -45, 0, 45, 90, 135, 180] {
            for rx in [-90, -45, 0, 45, 90] {
                out.push(Cube::new(mul(rot_x(deg(rx as f32)), rot_y(deg(ry as f32)))));
            }
        }
        out
    }

    #[test]
    fn every_orientation_holds_its_invariants() {
        for cube in orientations() {
            let problems = cube.verify();
            assert!(
                problems.is_empty(),
                "cube at {:?} reported {:?}",
                cube.m,
                problems
            );
        }
    }

    #[test]
    fn a_face_on_cube_matches_the_drawn_geometry() {
        let poly = Cube::facing(Face::Front).polygon(Face::Front);
        let xs: Vec<f32> = poly.iter().map(|(x, _)| *x).collect();
        let ys: Vec<f32> = poly.iter().map(|(_, y)| *y).collect();
        let min_x = xs.iter().cloned().fold(f32::MAX, f32::min);
        let max_x = xs.iter().cloned().fold(f32::MIN, f32::max);
        let min_y = ys.iter().cloned().fold(f32::MAX, f32::min);
        let max_y = ys.iter().cloned().fold(f32::MIN, f32::max);
        assert!((min_x - 25.0).abs() < 0.01, "front face left edge {min_x}");
        assert!(
            (max_x - 131.0).abs() < 0.01,
            "front face right edge {max_x}"
        );
        assert!((min_y - 25.0).abs() < 0.01, "front face top edge {min_y}");
        assert!(
            (max_y - 131.0).abs() < 0.01,
            "front face bottom edge {max_y}"
        );
    }

    #[test]
    fn the_back_face_is_drawn_smaller() {
        let poly = Cube::facing(Face::Front).polygon(Face::Back);
        let xs: Vec<f32> = poly.iter().map(|(x, _)| *x).collect();
        let width = xs.iter().cloned().fold(f32::MIN, f32::max)
            - xs.iter().cloned().fold(f32::MAX, f32::min);
        assert!(
            (width - 65.2).abs() < 0.2,
            "back face should be drawn at 0.615 of the front, got {width}"
        );
    }

    #[test]
    fn the_drag_neighbours_agree_with_the_sheet_ring() {
        let cube = Cube::facing(Face::Front);
        assert_eq!(cube.neighbour(Direction::Left), Face::Right);
        assert_eq!(cube.neighbour(Direction::Right), Face::Left);
        assert_eq!(cube.neighbour(Direction::Up), Face::Bottom);
        assert_eq!(cube.neighbour(Direction::Down), Face::Top);
    }

    #[test]
    fn turning_to_a_face_makes_that_face_active() {
        for face in Face::ALL {
            let cube = Cube::new(snap_to(IDENTITY, face));
            assert_eq!(cube.active(), face, "snap_to({face:?}) landed elsewhere");
        }
    }

    #[test]
    fn the_face_you_stand_on_is_never_dashed() {
        let cube = Cube::facing(Face::Front);
        let edges = cube.edges();
        assert_eq!(edges.iter().filter(|e| e.hidden).count(), 8);
        assert_eq!(edges.iter().filter(|e| e.active).count(), 4);
        for (i, (_, _, f1, f2)) in EDGES.iter().enumerate() {
            if *f1 == Face::Front || *f2 == Face::Front {
                assert!(
                    !edges[i].hidden,
                    "edge {i} bounds the active face but is dashed"
                );
            }
        }
    }

    #[test]
    fn the_section_sits_inside_the_active_face() {
        let cube = Cube::new(mul(rot_x(deg(20.0)), rot_y(deg(35.0))));
        let outer = cube.polygon(cube.active());
        let section = cube.section();
        assert!(section.starts_with('M') && section.ends_with('Z'));
        let inner_area = {
            let body = &section[1..section.len() - 1];
            let pts: Vec<(f32, f32)> = body
                .split('L')
                .filter_map(|p| {
                    let mut it = p.split_whitespace();
                    Some((it.next()?.parse().ok()?, it.next()?.parse().ok()?))
                })
                .collect();
            polygon_area(&pts)
        };
        assert!(inner_area < polygon_area(&outer));
        assert!(inner_area > 0.0);
    }

    #[test]
    fn an_axis_pointing_at_the_reader_collapses_to_a_symbol() {
        let triad = Cube::facing(Face::Front).triad();
        assert!(
            triad[2].collapsed && triad[2].toward,
            "front axis should collapse"
        );
        assert!(!triad[0].collapsed, "right axis should be drawn");
        assert!(!triad[1].collapsed, "top axis should be drawn");
    }

    #[test]
    fn a_step_there_and_back_returns_from_face_on() {
        for face in Face::ALL {
            let m = face_matrix(face, IDENTITY);
            let cube = Cube::new(m);
            assert_eq!(cube.active(), face, "{face:?} is not face-on");
            for dir in [
                Direction::Left,
                Direction::Right,
                Direction::Up,
                Direction::Down,
            ] {
                let n = cube.neighbour(dir);
                let there = Cube::new(snap_to(m, n));
                assert_eq!(there.active(), n, "turning toward {n:?} did not land on it");
                let back = there.neighbour(dir.opposite());
                assert_eq!(
                    back, face,
                    "from {face:?} a drag {dir:?} and back landed on {back:?}"
                );
            }
        }
    }

    #[test]
    fn snapping_never_leaves_the_cube_rolled() {
        for cube in orientations() {
            for face in Face::ALL {
                let snapped = Cube::new(snap_to(cube.m, face));
                assert!(
                    is_canonical(snapped.m),
                    "snap_to({:?}) is not canonical: {:?}",
                    face,
                    snapped.m
                );
                assert_eq!(snapped.active(), face, "snap_to({face:?}) lost the face");
            }
        }
    }

    #[test]
    fn snapping_takes_the_nearest_roll_of_the_four() {
        let nearly = Cube::new(mul(rot_z(deg(4.0)), IDENTITY));
        let snapped = snap_to(nearly.m, Face::Front);
        assert!(
            same(snapped, IDENTITY),
            "a 4-degree roll snapped to {snapped:?} rather than square"
        );
        let nearly_three_quarters = Cube::new(mul(rot_z(deg(80.0)), IDENTITY));
        let snapped = snap_to(nearly_three_quarters.m, Face::Front);
        assert!(
            same(snapped, canonical(Face::Front, 1)),
            "an 80-degree roll should take the quarter-turn the short way"
        );
    }

    #[test]
    fn a_canonical_cube_looks_square() {
        for face in Face::ALL {
            for roll in 0..4 {
                assert!(
                    is_canonical(canonical(face, roll)),
                    "{face:?} roll {roll} is not square"
                );
            }
        }
    }

    #[test]
    fn every_frame_of_a_turn_is_a_real_rotation() {
        let from = IDENTITY;
        let to = face_matrix(Face::Right, IDENTITY);
        for i in 0..=20 {
            let t = i as f32 / 20.0;
            let m = slerp(from, to, t);
            let det = m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
                - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
                + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0]);
            assert!((det - 1.0).abs() < 1e-3, "frame {t} has determinant {det}");
            for row in m {
                let len = (row[0] * row[0] + row[1] * row[1] + row[2] * row[2]).sqrt();
                assert!(
                    (len - 1.0).abs() < 1e-3,
                    "frame {t} has a row of length {len}"
                );
            }
            assert!(
                Cube::new(m).verify().is_empty(),
                "frame {t} reported {:?}",
                Cube::new(m).verify()
            );
        }
        assert!(same(slerp(from, to, 1.0), to));
        assert!(same(slerp(from, to, 0.0), from));
    }

    #[test]
    fn a_turn_takes_the_short_way_round() {
        let from = face_matrix(Face::Front, IDENTITY);
        let to = face_matrix(Face::Back, IDENTITY);
        assert!(same(slerp(from, to, 1.0), to));
    }
}
