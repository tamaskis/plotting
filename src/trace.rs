use crate::colors::Color;
use crate::line_style::LineStyle;

/// Trace.
///
/// A trace represents a single data series to place on a plot.
///
/// # Constructing a trace
///
/// See the following constructors:
///
/// * [`Trace::new_2d`] - for 2D traces.
/// * [`Trace::new_3d`] - for 3D traces.
pub struct Trace {
    /// x-axis data.
    pub(crate) x: Vec<f64>,

    /// y-axis data.
    pub(crate) y: Vec<f64>,

    /// z-axis data.
    pub(crate) z: Option<Vec<f64>>,

    /// Trace name (appears in the legend.)
    pub(crate) name: Option<String>,

    /// Line color.
    pub(crate) line_color: Option<Color>,

    /// Line width.
    pub(crate) line_width: Option<f64>,

    /// Line style.
    pub(crate) line_style: Option<LineStyle>,
}

impl Trace {
    /// Constructor for a 2D trace.
    ///
    /// # Arguments
    ///
    /// * `x` - x-axis data.
    /// * `y` - y-axis data.
    ///
    /// # Returns
    ///
    /// Trace.
    ///
    /// # Example
    ///
    /// ```
    /// use plotting::{Color, LineStyle, NamedColor, Trace};
    ///
    /// let trace = Trace::new_2d([1.0, 2.0, 3.0], [4.0, 5.0, 6.0])
    ///     .name("My Trace")
    ///     .line_color(Color::named(NamedColor::Red))
    ///     .line_width(2.0)
    ///     .line_style(LineStyle::Dot);
    /// ```
    pub fn new_2d(x: impl Into<Vec<f64>>, y: impl Into<Vec<f64>>) -> Trace {
        Trace {
            x: x.into(),
            y: y.into(),
            z: None,
            name: None,
            line_color: None,
            line_width: None,
            line_style: None,
        }
    }

    /// Constructor for a 3D trace.
    ///
    /// # Arguments
    ///
    /// * `x` - x-axis data.
    /// * `y` - y-axis data.
    /// * `z` - z-axis data.
    ///
    /// # Returns
    ///
    /// Trace.
    ///
    /// # Example
    ///
    /// ```
    /// use plotting::{Color, LineStyle, NamedColor, Trace};
    ///
    /// let trace = Trace::new_3d([1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0])
    ///     .name("My Trace")
    ///     .line_color(Color::named(NamedColor::Red))
    ///     .line_width(2.0)
    ///     .line_style(LineStyle::Dot);
    /// ```
    pub fn new_3d(x: impl Into<Vec<f64>>, y: impl Into<Vec<f64>>, z: impl Into<Vec<f64>>) -> Trace {
        Trace {
            x: x.into(),
            y: y.into(),
            z: Some(z.into()),
            name: None,
            line_color: None,
            line_width: None,
            line_style: None,
        }
    }

    /// Set the name of this trace.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the trace (i.e. appears in the legend).
    ///
    /// # Returns
    ///
    /// The trace with the updated name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the line color for this trace.
    ///
    /// # Arguments
    ///
    /// * `line_color` - Line color.
    ///
    /// # Returns
    ///
    /// The trace with the updated line color.
    pub fn line_color(mut self, line_color: Color) -> Self {
        self.line_color = Some(line_color);
        self
    }

    /// Set the line width for this trace.
    ///
    /// # Arguments
    ///
    /// * `line_width` - Line width.
    ///
    /// # Returns
    ///
    /// The trace with the updated line width.
    pub fn line_width(mut self, line_width: f64) -> Self {
        self.line_width = Some(line_width);
        self
    }

    /// Set the line style for this trace.
    ///
    /// # Arguments
    ///
    /// * `line_style` - Line style.
    ///
    /// # Returns
    ///
    /// The trace with the updated line style.
    pub fn line_style(mut self, line_style: LineStyle) -> Self {
        self.line_style = Some(line_style);
        self
    }
}
