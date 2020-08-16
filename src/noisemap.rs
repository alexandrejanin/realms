use noise::{NoiseFn, Perlin};
use rand::{RngCore, rngs::StdRng, SeedableRng};

pub struct NoiseParameters {
    pub scale: u8,
    pub octaves: usize,
    pub persistence: f64,
    pub lacunarity: f64,
}

pub struct FalloffParameters {
    pub a: f64,
    pub b: f64,
    pub multiplier: f64,
}

pub struct NoiseMap {
    pub map: Vec<f64>,
    pub min: f64,
    pub max: f64,
}

impl NoiseMap {
    pub fn normalized(&self, value: f64) -> f64 {
        (value - self.min) / (self.max - self.min)
    }

    pub(crate) fn new(seed: u64, width: usize, height: usize, parameters: &NoiseParameters) -> NoiseMap {
        let mut random = StdRng::seed_from_u64(seed);
        let perlin = Perlin::new();

        let mut map = Vec::with_capacity(width * height);

        let octave_offsets: Vec<(u32, u32)> = (0..parameters.octaves).into_iter().map(|_| (random.next_u32(), random.next_u32())).collect();

        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;

        for y in 0..height {
            for x in 0..width {
                let mut amplitude = 1.0;
                let mut frequency = 1.0;
                let mut value = 0.0;

                for i in 0..parameters.octaves {
                    let sample_x = (x as f64 - width as f64 / 2.0 + octave_offsets[i].0 as f64) / parameters.scale as f64 * frequency;
                    let sample_y = (y as f64 - height as f64 / 2.0 + octave_offsets[i].1 as f64) / parameters.scale as f64 * frequency;

                    let sample = perlin.get([sample_x, sample_y]);
                    value += amplitude * sample;
                    amplitude *= parameters.persistence;
                    frequency *= parameters.lacunarity;
                }

                if value < min { min = value }
                if value > max { max = value }

                map.push(value);
            }
        }

        NoiseMap {
            map,
            min,
            max,
        }
    }
    pub fn new_with_falloff(seed: u64, width: usize, height: usize, parameters: &NoiseParameters, falloff: &FalloffParameters) -> NoiseMap {
        let mut map = Self::new(seed, width, height, parameters);

        for y in 0..height {
            for x in 0..width {
                let i = (x as f64 / width as f64 * 2.0 - 1.0).abs();
                let j = (y as f64 / height as f64 * 2.0 - 1.0).abs();

                let value = f64::max(i, j);

                let falloff_value = Self::falloff(value, falloff.a, falloff.b);
                map.map[y * width + x] -= (map.max - map.min) * falloff.multiplier * falloff_value;
            }
        }

        map
    }

    fn falloff(value: f64, a: f64, b: f64) -> f64 {
        value.powf(a) / (value.powf(a) + (b - b * value).powf(a))
    }
}

