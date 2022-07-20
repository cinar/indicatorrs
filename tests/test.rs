use indicatorrs::indicator::trend::sma;

#[test]
fn test_sma() {
    let values = [1.0, 2.0, 3.0, 4.0];
    let expected = [0.0, 1.5, 2.5, 3.5];
    let period = 2;

    let average = sma(period, &values);
    assert_eq!(&average[..], &expected[..]);
}
