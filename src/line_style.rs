use plotly::common::DashType;

/// Line style.
///
/// # Note
///
/// This enum is a direct re-implementation of the [`DashType`] enum from the [`plotly`] crate (Ref.
/// \[1\]). As such, we have included the license of the [`plotly`] crate in the
/// [`src/plotly_licenses`](https://github.com/tamaskis/plotting/tree/main/src/plotly_licenses/LICENSE)
/// folder.
///
/// # References
///
/// * \[1\] <https://docs.rs/plotly/latest/plotly/common/enum.DashType.html>
#[derive(Clone, Copy)]
pub enum LineStyle {
    /// Solid line.
    Solid,

    /// Dotted line.
    Dot,

    /// Dashed line.
    Dash,

    /// Long dashed line.
    LongDash,

    /// Dashed line with dots.
    DashDot,

    /// Long dashed line with dots.
    LongDashDot,
}

impl From<LineStyle> for DashType {
    fn from(style: LineStyle) -> Self {
        match style {
            LineStyle::Solid => DashType::Solid,
            LineStyle::Dot => DashType::Dot,
            LineStyle::Dash => DashType::Dash,
            LineStyle::LongDash => DashType::LongDash,
            LineStyle::DashDot => DashType::DashDot,
            LineStyle::LongDashDot => DashType::LongDashDot,
        }
    }
}
