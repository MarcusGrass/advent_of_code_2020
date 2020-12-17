use std::ops::Range;

pub fn solve_both(session: &str) {
    let lines = vec![
        String::from(".#."),
        String::from("..#"),
        String::from("###")
    ];

    let lines = crate::util::fetch_lines(17, session);
    let mut grid = to_matrix(&lines);
    grid.expand();
    solve_first(grid.clone());
    let mut hyper = to_hyper(&lines);
    hyper.expand();
    solve_second(&hyper);
}

fn solve_first(grid: Grid) {
    let mut next = grid.clone();
    for _p in 0..6 {
        let mut next_perm = Vec::new();
        for i in next.xy_range() {
            let mut row = Vec::new();
            for j in next.xy_range() {
                let mut col = Vec::new();
                for k in next.z_range() {
                    let n = next.neighbours(i as i32, j as i32, k as i32);
                    let state = next.get(i, j, k);
                    if state {
                        if n == 2 || n == 3 {
                            col.push(state);
                        } else {
                            col.push(false)
                        }
                    } else {
                        if n == 3 {
                            col.push(true);
                        } else {
                            col.push(state);
                        }
                    }
                }
                row.push(col);
            }
            next_perm.push(row);
        }
        next = Grid{matrix: next_perm.clone()};
        /*
        for matrix in &next.matrix {
            println!("{:?}", matrix);
        }
        println!("{:?}", "");

         */
        next.expand();
    }
    println!("17.1 = {:?}", next.num_active());
}

fn solve_second(hyper: &HyperGrid) {
    let mut next = hyper.clone();
    for _p in 0..6 {
        let mut next_perm = Vec::new();
        for i in next.xy_range() {
            let mut row = Vec::new();
            for j in next.xy_range() {
                let mut sub = Vec::new();
                for k in next.z_range() {
                    let mut col = Vec::new();
                    for l in next.w_range() {
                        let n = next.neighbours(i as i32, j as i32, k as i32, l as i32);
                        let state = next.get(i, j, k, l);
                        if state {
                            if n == 2 || n == 3 {
                                col.push(state);
                            } else {
                                col.push(false)
                            }
                        } else {
                            if n == 3 {
                                col.push(true);
                            } else {
                                col.push(state);
                            }
                        }
                    }
                    sub.push(col);
                }
                row.push(sub);
            }
            next_perm.push(row);
        }
        next = HyperGrid{matrix: next_perm.clone()};
        /*
        for matrix in &next.matrix {
            println!("{:?}", matrix);
        }
        println!("{:?}", "");

         */
        next.expand();
    }
    println!("17.2 = {:?}", next.num_active());
}

fn to_matrix(lines: &Vec<String>) -> Grid {
    let mut mat = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        for char in line.chars() {
            let mut col = Vec::new();
            if char == '.' {
                col.push(false);
            } else {
                col.push(true);
            }
            row.push(col);
        }
        mat.push(row);
    }
    Grid{matrix: mat}
}

fn to_hyper(lines: &Vec<String>) -> HyperGrid {
    let mut mat = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        for char in line.chars() {
            let mut col = Vec::new();
            let mut sub = Vec::new();
            if char == '.' {
                col.push(false);
            } else {
                col.push(true);
            }
            sub.push(col);
            row.push(sub);
        }
        mat.push(row);
    }
    HyperGrid{matrix: mat}
}

#[derive(Debug, Clone)]
struct Grid {
    matrix: Vec<Vec<Vec<bool>>>
}

impl Grid {
    fn num_active(&self) -> usize {
        let mut count = 0;
        for matrix in &self.matrix {
            for row in matrix {
                for v in row {
                    if *v {count += 1}
                }
            }
        }
        count
    }

    fn get(&self, x: usize, y: usize, z: usize) -> bool {
        self.matrix[x][y][z]
    }

    fn get_or_default(&self, x: i32, y: i32, z: i32) -> bool {
        if x < 0 || y < 0 || z < 0 {
            return false;
        }
        *self.matrix.get(x as usize)
            .and_then(|m| m.get( y as usize))
            .and_then(|m| m.get(z as usize))
            .unwrap_or(&false)
    }

    fn xy_range(&self) -> Range<usize> {
        0..self.matrix.len()
    }

    fn z_range(&self) -> Range<usize> {
        0..self.matrix[0][0].len()
    }

    fn neighbours(&self, x: i32, y: i32, z: i32) -> usize {
        let mut n = 0;
        for i in -1..2 {
            for j in -1..2 {
                for k in -1..2 {
                    if i == 0 && j == 0 && k == 0 {
                        continue;
                    }
                    n += if self.get_or_default(x + i, y + j, z + k) {1} else {0};
                }
            }
        }
        n
    }

    fn expand(&mut self) {
        let len = self.matrix.len();
        let zlen = &self.matrix[0][0].len();
        self.matrix.insert(0, vec![vec![false; *zlen]; len]);
        self.matrix.push(vec![vec![false; *zlen]; len]);
        for i in 0..self.matrix.len() {
            self.matrix[i].insert(0, vec![false; *zlen]);
            self.matrix[i].push(vec![false; *zlen]);
            for j in 0..self.matrix[i].len() {
                self.matrix[i][j].insert(0, false);
                self.matrix[i][j].push(false);
            }

        }
    }

}

#[derive(Debug, Clone)]
struct HyperGrid {
    matrix: Vec<Vec<Vec<Vec<bool>>>>
}


impl HyperGrid {
    fn num_active(&self) -> usize {
        let mut count = 0;
        for matrix in &self.matrix {
            for row in matrix {
                for col in row {
                    for v in col {
                        if *v {count += 1}
                    }
                }
            }
        }
        count
    }

    fn get(&self, x: usize, y: usize, z: usize, w: usize) -> bool {
        self.matrix[x][y][z][w]
    }

    fn get_or_default(&self, x: i32, y: i32, z: i32, w: i32) -> bool {
        if x < 0 || y < 0 || z < 0 || w < 0 {
            return false;
        }
        *self.matrix.get(x as usize)
            .and_then(|m| m.get( y as usize))
            .and_then(|m| m.get(z as usize))
            .and_then(|m| m.get(w as usize))
            .unwrap_or(&false)
    }

    fn xy_range(&self) -> Range<usize> {
        0..self.matrix.len()
    }

    fn z_range(&self) -> Range<usize> {
        0..self.matrix[0][0].len()
    }

    fn w_range(&self) -> Range<usize> {
        0..self.matrix[0][0][0].len()
    }

    fn neighbours(&self, x: i32, y: i32, z: i32, w: i32) -> usize {
        let mut n = 0;
        for i in -1..2 {
            for j in -1..2 {
                for k in -1..2 {
                    for l in -1..2 {
                        if i == 0 && j == 0 && k == 0 && l == 0 {
                            continue;
                        }
                        n += if self.get_or_default(x + i, y + j, z + k, w + l) {1} else {0};
                    }

                }
            }
        }
        n
    }

    fn expand(&mut self) {
        let len = self.matrix.len();
        let zlen = &self.matrix[0][0].len();
        let wlen = &self.matrix[0][0][0].len();
        self.matrix.insert(0, vec![vec![vec![false; *wlen]; *zlen]; len]);
        self.matrix.push(vec![vec![vec![false; *wlen]; *zlen]; len]);
        for i in 0..self.matrix.len() {
            self.matrix[i].insert(0, vec![vec![false; *wlen]; *zlen]);
            self.matrix[i].push(vec![vec![false; *wlen]; *zlen]);
            for j in 0..self.matrix[i].len() {
                self.matrix[i][j].insert(0, vec![false; *wlen]);
                self.matrix[i][j].push(vec![false; *wlen]);
                for k in 0..self.matrix[i][j].len() {
                    self.matrix[i][j][k].insert(0, false);
                    self.matrix[i][j][k].push(false);
                }
            }

        }
    }

}

#[derive(Debug, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
    z: i32
}