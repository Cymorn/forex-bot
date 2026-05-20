use crate::structure_state::Trend;

pub fn trend_alignment(
    higher_tf: &Trend,
    lower_tf: &Trend,
) -> bool {

    higher_tf == lower_tf
}