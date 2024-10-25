use super::Fill;
use a1::A1;

#[allow(clippy::from_over_into)]
impl Into<A1> for Fill {
    fn into(self) -> A1 {
        a1::row_range(self.start_row, self.end_row().shift_up(1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_a1_finite() {
        let a1: A1 = Fill {
            amount: Some(3),
            start_row: 0.into(),
        }
        .into();
        assert_eq!("1:3", a1.to_string())
    }

    #[test]
    fn into_a1_infinite() {
        let a1: A1 = Fill {
            amount: None,
            start_row: 0.into(),
        }
        .into();
        assert_eq!("1:1000", a1.to_string())
    }
}
