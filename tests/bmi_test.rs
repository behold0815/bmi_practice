#[path = "../src/bmi.rs"]
mod bmi;

#[cfg(test)]
mod tests {
    use super::bmi::bmi_calculate;

    #[test]
    fn bmi_test() {
        assert_eq!(bmi_calculate(173, 68), 22.7);
    }
}
