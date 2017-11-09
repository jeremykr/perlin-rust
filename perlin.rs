// 2D Perlin noise implementation.
// Based on code from https://en.wikipedia.org/wiki/Perlin_noise
extern crate rand;
use std;

const TAU: f32 = std::f32::consts::PI * 2.0;

enum SmoothFunction {
    Linear,
    Smoothstep,
    Smootherstep,
    Smootheststep
}

pub struct PerlinNoise {
    g: Vec<Vec<Vec<f32>>>,
    smooth: SmoothFunction
}

impl PerlinNoise {
    pub fn new(size: usize) -> PerlinNoise {
        PerlinNoise { 
            g: random_gradient_grid(size),
            smooth: SmoothFunction::Smootherstep
        }
    }

    fn interp(&self, a0: f32, a1: f32, x: f32) -> f32 {
        let mut xc = clamp(x);
        xc = match self.smooth {
            SmoothFunction::Smoothstep => smoothstep(xc),
            SmoothFunction::Smootherstep => smootherstep(xc),
            SmoothFunction::Smootheststep => smootheststep(xc),
            _ => xc
        };
        a0 + xc * (a1 - a0)
    }

    fn dot_grid_gradient(&self, ix: usize, iy: usize, x: f32, y: f32) -> f32 {
        let dx = x - ix as f32;
        let dy = y - iy as f32;
        dx * self.g[iy][ix][0] + dy * self.g[iy][ix][1]
    }

    #[allow(dead_code)]
    pub fn set_smooth_function(&mut self, s: &str) -> bool {
        match s {
            "linear" => { self.smooth = SmoothFunction::Linear; true },
            "smooth" => { self.smooth = SmoothFunction::Smoothstep; true },
            "smoother" => { self.smooth = SmoothFunction::Smootherstep; true },
            "smoothest" => { self.smooth = SmoothFunction::Smootheststep; true },
            _ => false
        }
    }

    pub fn get(&self, x: f32, y: f32) -> f32 {
        let x0 = x.floor() as usize;
        let x1 = x0 + 1;
        let y0 = y.floor() as usize;
        let y1 = y0 + 1;

        let sx = x - x0 as f32;
        let sy = y - y0 as f32;

        let (mut n0, mut n1): (f32, f32);
        n0 = self.dot_grid_gradient(x0, y0, x, y);
        n1 = self.dot_grid_gradient(x1, y0, x, y);
        let ix0 = self.interp(n0, n1, sx);
        n0 = self.dot_grid_gradient(x0, y1, x, y);
        n1 = self.dot_grid_gradient(x1, y1, x, y);
        let ix1 = self.interp(n0, n1, sx);
        self.interp(ix0, ix1, sy)
    }
}

fn random_gradient_grid(n: usize) -> Vec<Vec<Vec<f32>>> {
    let mut g = Vec::new();
    let mut r: f32;
    for i in 0..n {
        g.push(Vec::new());
        for _ in 0..n {
            r = rand::random::<f32>() * TAU;
            g[i].push(vec![r.cos(), r.sin()]);
        }
    }
    g
}

fn smoothstep(x: f32) -> f32 { 
    x*x*(3.0-2.0*x) 
}

fn smootherstep(x: f32) -> f32 { 
    x*x*x*(x*(x*6.0-15.0)+10.0) 
}

fn smootheststep(x: f32) -> f32 { 
    -x*x*x*x*(2.0*x*(5.0*x*(2.0*x-7.0)+42.0)-35.0) 
}

fn clamp(x: f32) -> f32 {
    if x < 0.0      { 0.0 } 
    else if x > 1.0 { 1.0 }
    else            { x }
}
