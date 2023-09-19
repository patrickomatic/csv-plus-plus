use super::Expand;
use a1_notation::A1;

#[allow(clippy::from_over_into)]
impl Into<A1> for Expand {
    fn into(self) -> A1 {
        a1_notation::row_range(self.start_row, self.end_row().shift_up(1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_a1_finite() {
        let a1: A1 = Expand {
            amount: Some(3),
            start_row: 0.into(),
        }
        .into();
        assert_eq!("1:3", a1.to_string())
    }

    #[test]
    fn into_a1_infinite() {
        let a1: A1 = Expand {
            amount: None,
            start_row: 0.into(),
        }
        .into();
        assert_eq!("1:1000", a1.to_string())
    }
}
