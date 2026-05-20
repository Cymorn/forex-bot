use crate::structure_state::Trend;

#[derive(Debug, Clone, PartialEq)]
pub enum Bias {
    StrongBullish,
    Bullish,
    Neutral,
    Bearish,
    StrongBearish,
}

pub fn calculate_bias(
    d1: &Trend,
    h4: &Trend,
    h1: &Trend,
) -> Bias {

    let score =
        trend_score(d1) +
        trend_score(h4) +
        trend_score(h1);

    match score {
        3 => Bias::StrongBullish,
        2 => Bias::Bullish,
        1 => Bias::Bullish,
        0 => Bias::Neutral,
        -1 => Bias::Bearish,
        -2 => Bias::Bearish,
        -3 => Bias::StrongBearish,
        _ => Bias::Neutral,
    }
}

fn trend_score(trend: &Trend) -> i32 {
    match trend {
        Trend::Bullish => 1,
        Trend::Bearish => -1,
        Trend::Neutral => 0,
    }
}