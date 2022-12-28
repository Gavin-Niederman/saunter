pub fn lerp(a: f32, b: f32, t: f32) -> Result<f32, &'static str> {
    if t < 0.0 || t > 1.0 {
        return Err("t must be between 0 and 1");
    }

    Ok(a * t + b * (1.0 - t))
}

pub fn lerp_vec(list: Vec<(f32, f32)>, t: f32) -> Result<Vec<f32>, &'static str> {
    let mut result = Vec::new();

    for (a, b) in list {
        result.push(lerp(a, b, t)?);
    }

    Ok(result)
}
