pub struct FormattingUtils;

impl FormattingUtils {
    #[inline]
    pub fn format_money(amount: f64) -> String {
        let val = amount.abs();

        if val >= 1_000_000.0 {
            format!("{:.1}M", amount / 1_000_000.0)
        } else if val >= 1_000.0 {
            format!("{:.1}K", amount / 1_000.0)
        } else if val > -1_000.0 {
            format!("{:.2}", amount)
        } else {
            format!("{:.0}K", amount / 1_000.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_money_millions() {
        assert_eq!(FormattingUtils::format_money(1_000_000.0), "1.0M");
        assert_eq!(FormattingUtils::format_money(2_500_000.0), "2.5M");
        assert_eq!(FormattingUtils::format_money(999_999.99), "1000.0K");
        assert_eq!(FormattingUtils::format_money(-1_000_000.0), "-1.0M");
        assert_eq!(FormattingUtils::format_money(-2_500_000.0), "-2.5M");
        assert_eq!(FormattingUtils::format_money(-999_999.99), "-1000.0K");
    }

    #[test]
    fn test_format_money_thousands() {
        assert_eq!(FormattingUtils::format_money(1_000.0), "1.0K");
        assert_eq!(FormattingUtils::format_money(2_500.0), "2.5K");
        assert_eq!(FormattingUtils::format_money(999.99), "999.99");
        assert_eq!(FormattingUtils::format_money(-1_000.0), "-1.0K");
        assert_eq!(FormattingUtils::format_money(-2_500.0), "-2.5K");
        assert_eq!(FormattingUtils::format_money(-999.99), "-999.99");
    }

    #[test]
    fn test_format_money_small() {
        assert_eq!(FormattingUtils::format_money(123.45), "123.45");
        assert_eq!(FormattingUtils::format_money(999.0), "999.00");
        assert_eq!(FormattingUtils::format_money(-123.45), "-123.45");
        assert_eq!(FormattingUtils::format_money(-999.0), "-999.00");
    }
}
