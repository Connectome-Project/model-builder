use crate::{pattern::Pattern, ConnectionType};
use std::{iter::Peekable, vec::IntoIter};

#[allow(dead_code)]
pub async fn build_model<Pat: Pattern, D: From<Pat>>(
    _: ConnectionType,
    mut data_to_build: Peekable<IntoIter<Pat>>,
) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let pattern_so_far = Pat::default();
    if let Some(pattern_segment) = data_to_build.peek() {
        let _ = pattern_so_far.concat(pattern_segment.clone());
    }
    Ok(())
}
