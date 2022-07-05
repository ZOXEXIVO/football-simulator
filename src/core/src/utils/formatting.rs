pub struct FormattingUtils;

impl FormattingUtils {
    #[inline]
    pub fn short_money_str(val: f64) -> String {
        if val > 1_000_000.0 {
            format!("{0:.1}M", val / 1_000_000.0)
        } else {
            format!("{0}K", (val / 1_000.0) as f64)
        }
    }
}

