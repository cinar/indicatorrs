/// Calculates the Simple moving average (SMA).
///
/// # Arguments
///
/// * `period` window period.
/// * `values` values array.
///
/// # Examples
///
///  ```
/// use indicatorrs::indicator::trend::sma;
/// let values = vec![1.0, 2.0, 3.0, 4.0];
/// let average = sma(2, &values);
/// ```
pub fn sma(period: usize, values: &[f64]) -> Vec<f64> {
    let mut result = vec![0.0; values.len()];
    let mut sum: f64 = 0.0;

    for i in 0..values.len() {
        sum += values[i];

        if i >= period {
            sum -= values[i - period];
        }

        if i >= (period - 1) {
            result[i] = sum / (period as f64);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::sma;

    #[test]
    fn test_sma() {
        let values = [1.0, 2.0, 3.0, 4.0];
        let expected = [0.0, 1.5, 2.5, 3.5];
        let period = 2;

        let average = sma(period, &values);
        assert_eq!(&average[..], &expected[..]);
    }
}
