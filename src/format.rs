use derive_builder::Builder;

/// Figure format.
///
/// # Examples
///
/// ## Default format
///
/// ```
/// use plotting::Format;
///
/// let format = Format::default();
/// ```
///
/// ## Custom formatting
///
/// ```
/// use plotting::{Format, FormatBuilder};
///
/// let format: Format = FormatBuilder::default()
///     .title("y vs. x")
///     .x_label("x")
///     .y_label("y")
///     .width(800)
///     .height(600)
///     .build()
///     .unwrap();
/// ```
#[derive(Builder, Clone, Default)]
pub struct Format {
    /// Title.
    #[builder(setter(into, strip_option), default)]
    pub(crate) title: Option<String>,

    /// x-axis label.
    #[builder(setter(into, strip_option), default)]
    pub(crate) x_label: Option<String>,

    /// y-axis label.
    #[builder(setter(into, strip_option), default)]
    pub(crate) y_label: Option<String>,

    /// z-axis label.
    #[builder(setter(into, strip_option), default)]
    pub(crate) z_label: Option<String>,

    /// Width (in pixels).
    #[builder(setter(strip_option), default)]
    pub(crate) width: Option<usize>,

    /// Height (in pixels).
    #[builder(setter(strip_option), default)]
    pub(crate) height: Option<usize>,
}
