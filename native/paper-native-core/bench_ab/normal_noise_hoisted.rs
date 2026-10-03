use paper_native_core::perlin_noise::PerlinNoise;

pub const INPUT_FACTOR: f64 = 1.018_126_888_217_522_7;
const INLINE_AXIS_CACHE: usize = 32;

#[inline]
pub fn get_value(
    first: &PerlinNoise,
    second: &PerlinNoise,
    value_factor: f64,
    x: f64,
    y: f64,
    z: f64,
) -> f64 {
    let d = x * INPUT_FACTOR;
    let d1 = y * INPUT_FACTOR;
    let d2 = z * INPUT_FACTOR;
    (first.get_value_direct_math_wrap(x, y, z) + second.get_value_direct_math_wrap(d, d1, d2)) * value_factor
}

#[inline]
fn get_value_scaled_second(
    first: &PerlinNoise,
    second: &PerlinNoise,
    value_factor: f64,
    x: f64,
    y: f64,
    z: f64,
    second_x: f64,
    second_y: f64,
    second_z: f64,
) -> f64 {
    (first.get_value_direct_math_wrap(x, y, z)
        + second.get_value_direct_math_wrap(second_x, second_y, second_z))
        * value_factor
}

#[inline]
fn scaled_second(value: f64) -> f64 {
    value * INPUT_FACTOR
}

#[inline]
pub fn fill_positions(
    first: &PerlinNoise,
    second: &PerlinNoise,
    value_factor: f64,
    xs: &[f64],
    ys: &[f64],
    zs: &[f64],
    dst: &mut [f64],
) -> Result<(), &'static str> {
    if xs.len() != ys.len() || xs.len() != zs.len() {
        return Err("input lengths do not match");
    }
    if dst.len() != xs.len() {
        return Err("destination length does not match input length");
    }

    for index in 0..xs.len() {
        dst[index] = get_value_scaled_second(
            first,
            second,
            value_factor,
            xs[index],
            ys[index],
            zs[index],
            scaled_second(xs[index]),
            scaled_second(ys[index]),
            scaled_second(zs[index]),
        );
    }

    Ok(())
}

#[inline]
pub fn fill_scaled_positions(
    first: &PerlinNoise,
    second: &PerlinNoise,
    value_factor: f64,
    block_x: &[i32],
    block_y: &[i32],
    block_z: &[i32],
    xz_scale: f64,
    y_scale: f64,
    dst: &mut [f64],
) -> Result<(), &'static str> {
    if block_x.len() != block_y.len() || block_x.len() != block_z.len() {
        return Err("input lengths do not match");
    }
    if dst.len() != block_x.len() {
        return Err("destination length does not match input length");
    }

    for index in 0..block_x.len() {
        let noise_x = block_x[index] as f64 * xz_scale;
        let noise_y = block_y[index] as f64 * y_scale;
        let noise_z = block_z[index] as f64 * xz_scale;
        dst[index] = get_value_scaled_second(
            first,
            second,
            value_factor,
            noise_x,
            noise_y,
            noise_z,
            scaled_second(noise_x),
            scaled_second(noise_y),
            scaled_second(noise_z),
        );
    }

    Ok(())
}

#[inline]
pub fn fill_shifted_positions_in_place(
    first: &PerlinNoise,
    second: &PerlinNoise,
    value_factor: f64,
    shift_x_and_dst: &mut [f64],
    shift_y: &[f64],
    shift_z: &[f64],
    block_x: &[i32],
    block_y: &[i32],
    block_z: &[i32],
    xz_scale: f64,
    y_scale: f64,
) -> Result<(), &'static str> {
    if shift_x_and_dst.len() != shift_y.len()
        || shift_x_and_dst.len() != shift_z.len()
        || shift_x_and_dst.len() != block_x.len()
        || shift_x_and_dst.len() != block_y.len()
        || shift_x_and_dst.len() != block_z.len()
    {
        return Err("input lengths do not match");
    }

    for index in 0..shift_x_and_dst.len() {
        let noise_x = block_x[index] as f64 * xz_scale + shift_x_and_dst[index];
        let noise_y = block_y[index] as f64 * y_scale + shift_y[index];
        let noise_z = block_z[index] as f64 * xz_scale + shift_z[index];
        shift_x_and_dst[index] = get_value_scaled_second(
            first,
            second,
            value_factor,
            noise_x,
            noise_y,
            noise_z,
            scaled_second(noise_x),
            scaled_second(noise_y),
            scaled_second(noise_z),
        );
    }

    Ok(())
}

#[inline]
pub fn fill_shift_positions(
    first: &PerlinNoise,
    second: &PerlinNoise,
    value_factor: f64,
    block_x: &[i32],
    block_y: &[i32],
    block_z: &[i32],
    dst: &mut [f64],
) -> Result<(), &'static str> {
    if block_x.len() != block_y.len() || block_x.len() != block_z.len() {
        return Err("input lengths do not match");
    }
    if dst.len() != block_x.len() {
        return Err("destination length does not match input length");
    }

    for index in 0..block_x.len() {
        let noise_x = block_x[index] as f64 * 0.25;
        let noise_y = block_y[index] as f64 * 0.25;
        let noise_z = block_z[index] as f64 * 0.25;
        dst[index] = get_value_scaled_second(
            first,
            second,
            value_factor,
            noise_x,
            noise_y,
            noise_z,
            scaled_second(noise_x),
            scaled_second(noise_y),
            scaled_second(noise_z),
        ) * 4.0;
    }

    Ok(())
}

#[inline]
pub fn fill_shift_a(
    first: &PerlinNoise,
    second: &PerlinNoise,
    value_factor: f64,
    block_x: &[i32],
    block_z: &[i32],
    dst: &mut [f64],
) -> Result<(), &'static str> {
    if block_x.len() != block_z.len() {
        return Err("input lengths do not match");
    }
    if dst.len() != block_x.len() {
        return Err("destination length does not match input length");
    }

    for index in 0..block_x.len() {
        let noise_x = block_x[index] as f64 * 0.25;
        let noise_z = block_z[index] as f64 * 0.25;
        dst[index] = get_value_scaled_second(
            first,
            second,
            value_factor,
            noise_x,
            0.0,
            noise_z,
            scaled_second(noise_x),
            0.0,
            scaled_second(noise_z),
        ) * 4.0;
    }

    Ok(())
}

#[inline]
pub fn fill_shift_b(
    first: &PerlinNoise,
    second: &PerlinNoise,
    value_factor: f64,
    block_x: &[i32],
    block_z: &[i32],
    dst: &mut [f64],
) -> Result<(), &'static str> {
    if block_x.len() != block_z.len() {
        return Err("input lengths do not match");
    }
    if dst.len() != block_x.len() {
        return Err("destination length does not match input length");
    }

    for index in 0..block_x.len() {
        let noise_x = block_z[index] as f64 * 0.25;
        let noise_y = block_x[index] as f64 * 0.25;
        dst[index] = get_value_scaled_second(
            first,
            second,
            value_factor,
            noise_x,
            noise_y,
            0.0,
            scaled_second(noise_x),
            scaled_second(noise_y),
            0.0,
        ) * 4.0;
    }

    Ok(())
}

#[inline]
pub fn fill_vertical(
    first: &PerlinNoise,
    second: &PerlinNoise,
    value_factor: f64,
    x: f64,
    start_y: f64,
    y_step: f64,
    z: f64,
    dst: &mut [f64],
) {
    let second_x = scaled_second(x);
    let second_z = scaled_second(z);
    for (index, value) in dst.iter_mut().enumerate() {
        let noise_y = start_y + index as f64 * y_step;
        *value = get_value_scaled_second(
            first,
            second,
            value_factor,
            x,
            noise_y,
            z,
            second_x,
            scaled_second(noise_y),
            second_z,
        );
    }
}

#[inline]
pub fn fill_cell(
    first: &PerlinNoise,
    second: &PerlinNoise,
    value_factor: f64,
    cell_width: usize,
    cell_height: usize,
    base_x: f64,
    base_y: f64,
    base_z: f64,
    xz_scale: f64,
    y_scale: f64,
    dst: &mut [f64],
) -> Result<(), &'static str> {
    let expected = cell_width
        .checked_mul(cell_width)
        .and_then(|value| value.checked_mul(cell_height))
        .ok_or("cell dimensions overflow")?;
    if dst.len() != expected {
        return Err("destination length does not match cell dimensions");
    }

    if cell_width <= INLINE_AXIS_CACHE {
        let mut noise_x_axis = [0.0; INLINE_AXIS_CACHE];
        let mut scaled_noise_x_axis = [0.0; INLINE_AXIS_CACHE];
        let mut noise_z_axis = [0.0; INLINE_AXIS_CACHE];
        let mut scaled_noise_z_axis = [0.0; INLINE_AXIS_CACHE];
        for axis in 0..cell_width {
            let noise_x = (base_x + axis as f64) * xz_scale;
            let noise_z = (base_z + axis as f64) * xz_scale;
            noise_x_axis[axis] = noise_x;
            scaled_noise_x_axis[axis] = noise_x * INPUT_FACTOR;
            noise_z_axis[axis] = noise_z;
            scaled_noise_z_axis[axis] = noise_z * INPUT_FACTOR;
        }

        let mut index = 0;
        for y in (0..cell_height).rev() {
            let noise_y = (base_y + y as f64) * y_scale;
            let scaled_noise_y = noise_y * INPUT_FACTOR;
            for x in 0..cell_width {
                let noise_x = noise_x_axis[x];
                let scaled_noise_x = scaled_noise_x_axis[x];
                for z in 0..cell_width {
                    dst[index] = get_value_scaled_second(
                        first,
                        second,
                        value_factor,
                        noise_x,
                        noise_y,
                        noise_z_axis[z],
                        scaled_noise_x,
                        scaled_noise_y,
                        scaled_noise_z_axis[z],
                    );
                    index += 1;
                }
            }
        }

        return Ok(());
    }

    let mut index = 0;
    for y in (0..cell_height).rev() {
        let noise_y = (base_y + y as f64) * y_scale;
        let second_y = scaled_second(noise_y);
        for x in 0..cell_width {
            let noise_x = (base_x + x as f64) * xz_scale;
            let second_x = scaled_second(noise_x);
            for z in 0..cell_width {
                let noise_z = (base_z + z as f64) * xz_scale;
                dst[index] = get_value_scaled_second(
                    first,
                    second,
                    value_factor,
                    noise_x,
                    noise_y,
                    noise_z,
                    second_x,
                    second_y,
                    scaled_second(noise_z),
                );
                index += 1;
            }
        }
    }

    Ok(())
}

