use cgmath::*;
use rand::prelude::*;

const NEIGHBORHOOD: &[Vector3<i32>] = &[
    vec3(1, 0, 0),
    vec3(1, 1, 0),
    vec3(0, 1, 0),
    vec3(-1, 1, 0),
    vec3(-1, 0, 0),
    vec3(-1, -1, 0),
    vec3(0, -1, 0),
    vec3(1, -1, 0),
    vec3(1, 0, 1),
    vec3(1, 1, 1),
    vec3(0, 1, 1),
    vec3(-1, 1, 1),
    vec3(-1, 0, 1),
    vec3(-1, -1, 1),
    vec3(0, -1, 1),
    vec3(1, -1, 1),
    vec3(1, 0, -1),
    vec3(1, 1, -1),
    vec3(0, 1, -1),
    vec3(-1, 1, -1),
    vec3(-1, 0, -1),
    vec3(-1, -1, -1),
    vec3(0, -1, -1),
    vec3(1, -1, -1),
    vec3(0, 0, 1),
    vec3(0, 0, -1),
];

pub struct CA {
    pub size: Vector3<usize>,
    pub grid: Vec<Vec<Vec<bool>>>,
    pub grid2: Vec<Vec<Vec<bool>>>,
}

impl CA {
    pub fn new(rng: &mut impl Rng, size: Vector3<usize>) -> Self {
        let grid: Vec<_> = (0..size.z)
            .map(|_| {
                (0..size.y).map(|_| (0..size.x).map(|_| rng.gen::<f32>() < 0.1).collect()).collect()
            })
            .collect();
        Self { size, grid: grid.clone(), grid2: grid }
    }

    fn contains_point(&self, point: Point3<i32>) -> bool {
        point.x >= 0
            && point.x < self.size.x as i32
            && point.y >= 0
            && point.y < self.size.y as i32
            && point.z >= 0
            && point.z < self.size.z as i32
    }

    pub fn update(&mut self) {
        for z in 0..self.size.z {
            for y in 0..self.size.y {
                for x in 0..self.size.x {
                    let cur_point = point3(x, y, z).cast::<i32>().unwrap();
                    let cur_state = self.grid[z][y][x];
                    let n = NEIGHBORHOOD
                        .iter()
                        .map(|dir| (cur_point + *dir))
                        .filter(|point| {
                            self.contains_point(*point)
                                && self.grid[point.z as usize][point.y as usize][point.x as usize]
                        })
                        .count();
                    self.grid2[z][y][x] = (!cur_state && (n == 6 || n == 8))
                        || (cur_state && (n == 5 || n == 6 || n == 7 || n == 13));
                }
            }
        }
        std::mem::swap(&mut self.grid, &mut self.grid2);
    }
}
