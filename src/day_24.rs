use std::{fmt::Debug, io, result, str::FromStr};

use nalgebra::{DMatrix, DVector};

use crate::fs::read_day;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Vec3 {
  x: i64,
  y: i64,
  z: i64,
}

impl Vec3 {
  fn new(x: i64, y: i64, z: i64) -> Vec3 {
    Vec3 { x, y, z }
  }

  fn to_tuple(&self) -> (i64, i64, i64) {
    (self.x, self.y, self.z)
  }
}

impl FromStr for Vec3 {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut parts = s.trim().split(',').map(|s| s.trim());
    let x = parts.next().unwrap().parse().unwrap();
    let y = parts.next().unwrap().parse().unwrap();
    let z = parts.next().unwrap().parse().unwrap();
    Ok(Vec3::new(x, y, z))
  }
}

impl Debug for Vec3 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {}, {})", self.x, self.y, self.z)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Hailstone {
  position: Vec3,
  velocity: Vec3,
}

impl Hailstone {
  fn new(position: Vec3, velocity: Vec3) -> Hailstone {
    Hailstone { position, velocity }
  }

  // x = xs + t * vx
  // y = ys + t * vy
  // Eliminating t
  // (x - xs) / vx = (y - ys) / vy
  // (x - xs) * vy = (y - ys) * vx
  // Lets get this in the form of ax + by = c
  // (vy) x - (vx) y = (vy * xs) - (vx * ys)
  fn standard_form_xy(&self) -> Vec3 {
    let a = self.velocity.y;
    let b = -self.velocity.x;
    let c = self.velocity.y * self.position.x - self.velocity.x * self.position.y;
    Vec3::new(a, b, c)
  }

  fn intersection_xy(&self, other: &Hailstone) -> Option<(f64, f64)> {
    let (a1, b1, c1) = self.standard_form_xy().to_tuple();
    let (a2, b2, c2) = other.standard_form_xy().to_tuple();
    let determinant = (a1 * b2 - a2 * b1) as f64;
    if determinant == 0f64 {
      return None;
    }
    let x = (b2 as f64 / determinant * c1 as f64) - (b1 as f64 / determinant * c2 as f64);
    let y = (a1 as f64 / determinant * c2 as f64) - (a2 as f64 / determinant * c1 as f64);
    Some((x, y))
  }
}

impl FromStr for Hailstone {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut parts = s.trim().split('@').map(|s| s.trim());
    let position = parts.next().unwrap().parse().unwrap();
    let velocity = parts.next().unwrap().parse().unwrap();
    Ok(Hailstone::new(position, velocity))
  }
}

fn parse_input() -> io::Result<Vec<Hailstone>> {
  let hailstones = read_day(24)?.map(|line| line.parse().unwrap()).collect();
  Ok(hailstones)
}

pub fn part_1() -> io::Result<usize> {
  let hailstones = parse_input()?;

  let bounding_box = (200000000000000f64, 400000000000000f64);

  let result = hailstones
    .iter()
    .enumerate()
    .flat_map(|(index, hs1)| {
      hailstones
        .iter()
        .skip(index + 1)
        .filter_map(|hs2| {
          hs1.intersection_xy(hs2).and_then(|(x, y)| {
            // Check if its in the past by comparing the sign of the difference with velocity
            let in_the_future = vec![hs1, hs2].iter().all(|hs| {
              (x - hs.position.x as f64) * hs.velocity.x as f64 >= 0f64
                && (y - hs.position.y as f64) * hs.velocity.y as f64 >= 0f64
            });

            let in_bounding_box = x >= bounding_box.0 as f64
              && x <= bounding_box.1 as f64
              && y >= bounding_box.0 as f64
              && y <= bounding_box.1 as f64;

            if in_the_future && in_bounding_box {
              Some((x, y))
            } else {
              None
            }
          })
        })
        .collect::<Vec<_>>()
    })
    .count();

  Ok(result)
}

pub fn part_2() -> io::Result<i64> {
  /*
  Xr + t1 * Vxr - X1 - t1 Vx1 = 0
  Yr + t1 * Vyr - Y1 - t1 Vy1 = 0
  Zr + t1 * Vzr - Z1 - t1 Vz1 = 0

  Xr + t2 * Vxr - X2 - t2 Vx2 = 0
  Yr + t2 * Vyr - Y2 - t2 Vy2 = 0
  Zr + t2 * Vzr - Z2 - t2 Vz2 = 0

  Xr + t3 * Vxr - X3 - t3 Vx3 = 0
  Yr + t3 * Vyr - Y3 - t3 Vy3 = 0
  Zr + t3 * Vzr - Z3 - t3 Vz3 = 0

  let input be [Xr, Yr, Zr, Vxr, Vyr, Vzr, t1, t2, t3]
   */

  let hailstones = parse_input()?;

  fn f(x: &DVector<f64>, hailstones: &Vec<Hailstone>) -> DVector<f64> {
    let f0 =
      // Xr + t1 * Vxr - X1 - t1 Vx1 = 0
      x[0] + x[6] * x[3] - (hailstones[0].position.x as f64) - x[6] * (hailstones[0].velocity.x as f64);

    let f1 =
      // Yr + t1 * Vyr - Y1 - t1 Vy1 = 0
      x[1] + x[6] * x[4] - (hailstones[0].position.y as f64) - x[6] * (hailstones[0].velocity.y as f64);

    let f2 =
      // Zr + t1 * Vzr - Z1 - t1 Vz1 = 0
      x[2] + x[6] * x[5] - (hailstones[0].position.z as f64) - x[6] * (hailstones[0].velocity.z as f64);

    let f3 =
      // Xr + t2 * Vxr - X2 - t2 Vx2 = 0
      x[0] + x[7] * x[3] - (hailstones[1].position.x as f64) - x[7] * (hailstones[1].velocity.x as f64);

    let f4 =
      // Yr + t2 * Vyr - Y2 - t2 Vy2 = 0
      x[1] + x[7] * x[4] - (hailstones[1].position.y as f64) - x[7] * (hailstones[1].velocity.y as f64);

    let f5 =
      // Zr + t2 * Vzr - Z2 - t2 Vz2 = 0
      x[2] + x[7] * x[5] - (hailstones[1].position.z as f64) - x[7] * (hailstones[1].velocity.z as f64);

    let f6 =
      // Xr + t3 * Vxr - X3 - t3 Vx3 = 0
      x[0] + x[8] * x[3] - (hailstones[2].position.x as f64) - x[8] * (hailstones[2].velocity.x as f64);

    let f7 =
      // Yr + t3 * Vyr - Y3 - t3 Vy3 = 0
      x[1] + x[8] * x[4] - (hailstones[2].position.y as f64) - x[8] * (hailstones[2].velocity.y as f64);

    let f8 =
      // Zr + t3 * Vzr - Z3 - t3 Vz3 = 0
      x[2] + x[8] * x[5] - (hailstones[2].position.z as f64) - x[8] * (hailstones[2].velocity.z as f64);

    DVector::from_vec(vec![f0, f1, f2, f3, f4, f5, f6, f7, f8])
  }

  fn jacobian(x: &DVector<f64>, hailstones: &Vec<Hailstone>) -> DMatrix<f64> {
    // get all zeroes 9x9
    let mut result = DMatrix::zeros(9, 9);

    fn set(
      matrix: &mut DMatrix<f64>,
      x: &DVector<f64>,
      row: usize,
      col: usize,
      index_of_t: usize,
      d: f64,
    ) {
      matrix[(row, col)] = 1f64;
      matrix[(row, col + 3)] = x[index_of_t];
      matrix[(row, index_of_t)] = x[col + 3] - d
    }

    set(&mut result, x, 0, 0, 6, hailstones[0].velocity.x as f64);
    set(&mut result, x, 1, 1, 6, hailstones[0].velocity.y as f64);
    set(&mut result, x, 2, 2, 6, hailstones[0].velocity.z as f64);

    set(&mut result, x, 3, 0, 7, hailstones[1].velocity.x as f64);
    set(&mut result, x, 4, 1, 7, hailstones[1].velocity.y as f64);
    set(&mut result, x, 5, 2, 7, hailstones[1].velocity.z as f64);

    set(&mut result, x, 6, 0, 8, hailstones[2].velocity.x as f64);
    set(&mut result, x, 7, 1, 8, hailstones[2].velocity.y as f64);
    set(&mut result, x, 8, 2, 8, hailstones[2].velocity.z as f64);

    result
  }

  fn newtons_method(
    initial_guess: DVector<f64>,
    hailstones: &Vec<Hailstone>,
    tolerance: f64,
    max_iterations: usize,
  ) -> DVector<f64> {
    let mut x = initial_guess;

    for _ in 0..max_iterations {
      let j = jacobian(&x, hailstones);
      let f_at_x = f(&x, &hailstones);
      let l2_norm = f_at_x.norm();
      if l2_norm < tolerance {
        break;
      }
      let j_inv = j.try_inverse().expect("Jacobian is not invertible");
      x = x - j_inv * f_at_x;
    }

    x
  }

  let initial_guess: DVector<f64> = DVector::from_vec(vec![
    -1f64, -2f64, -3f64, -4f64, -5f64, -6f64, -7f64, -8f64, -9f64,
  ]);

  let tolerance = 1e-6;

  let max_iterations = 1000;

  let root: DVector<f64> = newtons_method(initial_guess, &hailstones, tolerance, max_iterations);

  let position = Vec3::new(root[0] as i64, root[1] as i64, root[2] as i64);

  Ok(position.x + position.y + position.z)
}
