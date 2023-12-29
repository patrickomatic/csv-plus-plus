use super::Rgb;

fn safe_scale_to_1(value: u8) -> f32 {
    let val = f32::from(value) / 255.0;
    if val == std::f32::INFINITY {
        0.0
    } else {
        val
    }
}

impl From<&Rgb> for (f32, f32, f32) {
    fn from(value: &Rgb) -> (f32, f32, f32) {
        (
            safe_scale_to_1(value.r),
            safe_scale_to_1(value.g),
            safe_scale_to_1(value.b),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_rgb_tuple() {
        let rgb = &Rgb {
            r: 0,
            g: 255,
            b: 170,
        };
        let tuple: (f32, f32, f32) = rgb.into();

        assert_eq!(tuple.0, 0.0);
        assert_eq!(tuple.1, 1.0);
        assert_eq!(tuple.2, 0.666_666_7);
    }
}
