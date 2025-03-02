
pub fn calculate_lmsr_price(shares_yes: u64, shares_no: u64) -> (f64, f64) {
    let liquidity = (shares_yes + shares_no) as f64; // Dynamic liquidity

    if liquidity == 0.0 {
        return (0.5, 0.5); // Default to 50-50 probability
    }

    let exp_yes = f64::exp(shares_yes as f64 / liquidity);
    let exp_no = f64::exp(shares_no as f64 / liquidity);
    let sum_exp = exp_yes + exp_no;

    let price_yes = exp_yes / sum_exp;
    let price_no = exp_no / sum_exp;

    (price_yes, price_no)
}

