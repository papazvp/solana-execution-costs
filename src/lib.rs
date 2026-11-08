#[derive(Debug, PartialEq, Eq)]
pub struct Estimate {
    pub gross_lamports: i128,
    pub net_lamports: i128,
}

pub fn estimate(input: u64, output: u64, network_fee: u64, tip: u64) -> Estimate {
    let gross_lamports = i128::from(output) - i128::from(input);
    let net_lamports = gross_lamports - i128::from(network_fee) - i128::from(tip);
    Estimate {
        gross_lamports,
        net_lamports,
    }
}

pub fn format_sol(lamports: i128) -> String {
    let sign = if lamports < 0 { "-" } else { "" };
    let magnitude = lamports.unsigned_abs();
    format!(
        "{sign}{}.{:09}",
        magnitude / 1_000_000_000,
        magnitude % 1_000_000_000
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subtracts_network_fee_and_tip_from_gross_change() {
        let result = estimate(1_000_000_000, 1_002_000_000, 5_000, 200_000);
        assert_eq!(result.gross_lamports, 2_000_000);
        assert_eq!(result.net_lamports, 1_795_000);
    }

    #[test]
    fn reports_losses_even_when_gross_is_positive() {
        let result = estimate(1_000_000_000, 1_000_100_000, 5_000, 200_000);
        assert_eq!(result.net_lamports, -105_000);
    }

    #[test]
    fn handles_negative_gross_and_maximum_inputs_without_overflow() {
        let result = estimate(u64::MAX, 0, u64::MAX, u64::MAX);
        assert_eq!(result.gross_lamports, -i128::from(u64::MAX));
        assert_eq!(result.net_lamports, -3 * i128::from(u64::MAX));
    }

    #[test]
    fn formats_signed_amounts_without_floating_point() {
        assert_eq!(format_sol(1_795_000), "0.001795000");
        assert_eq!(format_sol(-105_000), "-0.000105000");
        assert_eq!(format_sol(0), "0.000000000");
    }
}
