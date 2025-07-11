use crate::format::Format;
use crate::trace::Trace;
use plotly::{
    Layout, Scatter, Scatter3D,
    common::{Line, Title},
    layout::Axis,
};
use std::fs;
use std::path::Path;

/// Three-dimensional plot.
pub struct Plot {
    /// Traces to plot on the figure.
    pub(crate) traces: Vec<Trace>,

    /// Formatting.
    pub(crate) format: Format,
}

impl Plot {
    /// Constructor.
    ///
    /// # Arguments
    ///
    /// * `traces` - Traces to plot on the figure.
    /// * `format` - Formatting.
    ///
    /// # Returns
    ///
    /// 3D plot.
    ///
    /// # Example
    ///
    /// ```
    /// use plotting::{Color, Format, FormatBuilder, LineStyle, NamedColor, Plot, Trace};
    ///
    /// // Define the traces.
    /// let trace_1 = Trace::new_3d([1.0, 2.0, 5.0], [1.0, 2.0, 3.0], [1.0, 2.0, 4.0])
    ///     .name("Trace 1")
    ///     .line_color(Color::named(NamedColor::Red))
    ///     .line_width(2.0)
    ///     .line_style(LineStyle::Dash);
    /// let trace_2 = Trace::new_3d([1.0, 2.0, 5.0], [3.0, 2.0, 1.0], [1.0, 2.0, 4.0])
    ///     .name("Trace 2")
    ///     .line_color(Color::named(NamedColor::Blue))
    ///     .line_width(2.0)
    ///     .line_style(LineStyle::Dot);
    ///
    /// // Plot formatting.
    /// let format: Format = FormatBuilder::default()
    ///     .title("z vs. x and y")
    ///     .x_label("x")
    ///     .y_label("y")
    ///     .z_label("z")
    ///     .width(800)
    ///     .height(600)
    ///     .build()
    ///     .unwrap();
    ///
    /// // Create the plot.
    /// let plot = Plot::new(vec![trace_1, trace_2], format);
    /// ```
    pub fn new(traces: Vec<Trace>, format: Format) -> Plot {
        Plot { traces, format }
    }

    /// Create a plotly `plot` for 3D.
    ///
    /// # Returns
    ///
    /// Plotly plot.
    pub fn plotly(&self) -> plotly::Plot {
        // Initialize the plot.
        let mut plot = plotly::Plot::new();

        // Iterate over traces.
        for trace in self.traces.as_slice() {
            // Line settings.
            let mut line = Line::new();
            if let Some(line_color) = &trace.line_color {
                line = line.color(line_color.to_plotly_rgba());
            }
            if let Some(line_width) = trace.line_width {
                line = line.width(line_width);
            }
            if let Some(line_style) = trace.line_style {
                line = line.dash(line_style.into());
            }

            // Data alone each axis.
            let x_data = trace.x.clone();
            let y_data = trace.y.clone();
            let z_data = trace.z.clone();

            // Create the plotly trace.
            let trace_plotly: Box<dyn plotly::Trace> = if let Some(z_data) = &z_data {
                Scatter3D::new(x_data, y_data, z_data.clone()).line(line)
            } else {
                Scatter::new(x_data, y_data).line(line)
            };

            // Add the trace to the plot.
            plot.add_trace(trace_plotly);
        }

        // x-axis label.
        let x_label_plotly: Axis = if let Some(x_label) = &self.format.x_label {
            Axis::new().title(x_label)
        } else {
            Axis::new()
        };

        // y-axis label.
        let y_label_plotly: Axis = if let Some(y_label) = &self.format.y_label {
            Axis::new().title(y_label)
        } else {
            Axis::new()
        };

        // z-axis label.
        let z_label_plotly: Option<Axis> = self
            .format
            .z_label
            .as_ref()
            .map(|z_label| Axis::new().title(z_label));

        // Plot title.
        let title_plotly: Title = if let Some(title) = &self.format.title {
            Title::with_text(title)
        } else {
            Title::new()
        };

        // Set the plot layout.
        let mut layout = Layout::new()
            .x_axis(x_label_plotly)
            .y_axis(y_label_plotly)
            .title(title_plotly);
        if let Some(z_axis) = z_label_plotly {
            layout = layout.z_axis(z_axis);
        }
        if let Some(width) = self.format.width {
            layout = layout.width(width);
        }
        if let Some(height) = self.format.height {
            layout = layout.height(height);
        }
        plot.set_layout(layout);

        plot
    }

    /// Show the plot (opens the plot in a web browser).
    ///
    /// # Example
    ///
    /// ```
    /// use plotting::{quick_plot_3d, Plot};
    ///
    /// // Create a quick 3D plot with a single trace.
    /// let plot: Plot = quick_plot_3d([1.0, 2.0, 10.0], [1.0, 4.0, 9.0], [2.0, 5.0, 10.0]);
    ///
    /// // Show the plot (will open in a web browser).
    /// plot.show();
    /// ```
    pub fn show(&self) {
        self.plotly().show();
    }

    /// Save the plot to a standalone HTML file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the HTML file.
    ///
    /// ```
    /// # fn main() -> std::io::Result<()> {
    /// use plotting::{quick_plot_3d, Plot};
    ///
    /// // Create a quick 3D plot with a single trace.
    /// let plot: Plot = quick_plot_3d([1.0, 2.0, 10.0], [1.0, 4.0, 9.0], [2.0, 5.0, 10.0]);
    ///
    /// // Save the plot to an HTML file.
    /// plot.save_html("folder/file.html")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn save_html<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let path = path.as_ref();
        let html_str = self.plotly().to_html();
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        fs::write(path, html_str)
    }

    /// Save the plot to an HTML file meant to be used "in-line" in another HTML file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the HTML file.
    ///
    /// ```
    /// # fn main() -> std::io::Result<()> {
    /// use plotting::{quick_plot_3d, Plot};
    ///
    /// // Create a quick 3D plot with a single trace.
    /// let plot: Plot = quick_plot_3d([1.0, 2.0, 10.0], [1.0, 4.0, 9.0], [2.0, 5.0, 10.0]);
    ///
    /// // Save the plot to an HTML file.
    /// plot.save_inline_html("folder/file.html")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn save_inline_html<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let path = path.as_ref();
        let html_str = self.plotly().to_inline_html(None);
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        fs::write(path, html_str)
    }
}
