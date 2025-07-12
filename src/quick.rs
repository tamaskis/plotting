use crate::figure::Figure;
use crate::format::FormatBuilder;
use crate::trace::Trace;

/// Quickly create a 2D plot.
///
/// # Arguments
///
/// * `x` - x-axis data.
/// * `y` - y-axis data.
///
/// # Returns
///
/// Figure.
///
/// # Example
///
/// ```
/// use plotting::{quick_plot_2d, Figure};
///
/// // Plot y = x^2.
/// let fig: Figure = quick_plot_2d([1.0, 2.0, 3.0], [1.0, 4.0, 9.0]);
/// ```
///
/// ## Additional examples
///
/// Additional examples can be found at
/// <https://tamaskis.github.io/plotting/plot_2d/quick_2d_plot.html>.
pub fn quick_plot_2d(x: impl Into<Vec<f64>>, y: impl Into<Vec<f64>>) -> Figure {
    // Create the trace.
    let trace = Trace::new_2d(x.into(), y.into());

    // Formatting.
    let format = FormatBuilder::default().build().unwrap();

    // Create and return the figure.
    Figure::new(vec![trace], format)
}

/// Quickly create a 2D plot with axis labels and a title.
///
/// # Arguments
///
/// * `x` - x-axis data.
/// * `y` - y-axis data.
/// # `x_label` - x-axis label.
/// * `y_label` - y-axis label.
/// * `title` - Title.
///
/// # Returns
///
/// Figure.
///
/// # Example
///
/// ```
/// use plotting::{quick_plot_2d_with_labels, Figure};
///
/// // Plot y = x^2.
/// let fig: Figure = quick_plot_2d_with_labels(
///     [1.0, 2.0, 3.0],
///     [1.0, 4.0, 9.0],
///     "x",
///     "y",
///     "y = x^2",
/// );
/// ```
///
/// ## Additional examples
///
/// Additional examples can be found at
/// <https://tamaskis.github.io/plotting/plot_2d/quick_2d_plot_with_labels.html>.
pub fn quick_plot_2d_with_labels(
    x: impl Into<Vec<f64>>,
    y: impl Into<Vec<f64>>,
    x_label: impl Into<String>,
    y_label: impl Into<String>,
    title: impl Into<String>,
) -> Figure {
    // Create the trace.
    let trace = Trace::new_2d(x.into(), y.into());

    // Formatting.
    let format = FormatBuilder::default()
        .x_label(x_label.into())
        .y_label(y_label.into())
        .title(title.into())
        .build()
        .unwrap();

    // Create and return the figure.
    Figure::new(vec![trace], format)
}

/// Quickly create a 3D plot.
///
/// # Arguments
///
/// * `x` - x-axis data.
/// * `y` - y-axis data.
/// * `z` - z-axis data.
///
/// # Returns
///
/// Figure.
///
/// # Example
///
/// ```
/// use plotting::{quick_plot_3d, Figure};
///
/// let fig: Figure = quick_plot_3d([1.0, 2.0, 10.0], [1.0, 4.0, 9.0], [2.0, 5.0, 10.0]);
/// ```
///
/// ## Additional examples
///
/// Additional examples can be found at
/// <https://tamaskis.github.io/plotting/plot_3d/quick_3d_plot.html>.
pub fn quick_plot_3d(
    x: impl Into<Vec<f64>>,
    y: impl Into<Vec<f64>>,
    z: impl Into<Vec<f64>>,
) -> Figure {
    // Create the trace.
    let trace = Trace::new_3d(x.into(), y.into(), z.into());

    // Formatting.
    let format = FormatBuilder::default().build().unwrap();

    // Create and return the figure.
    Figure::new(vec![trace], format)
}

/// Quickly create a 3D plot with axis labels and a title.
///
/// # Arguments
///
/// * `x` - x-axis data.
/// * `y` - y-axis data.
/// * `z` - z-axis data.
/// # `x_label` - x-axis label.
/// * `y_label` - y-axis label.
/// * `z_label` - z-axis label.
/// * `title` - Title.
///
/// # Returns
///
/// Figure.
///
/// ## Example
///
/// ```
/// use plotting::{quick_plot_3d_with_labels, Figure};
///
/// let fig: Figure = quick_plot_3d_with_labels(
///     [1.0, 2.0, 10.0],
///     [1.0, 4.0, 9.0],
///     [2.0, 5.0, 10.0],
///     "x",
///     "y",
///     "z",
///     "z vs. x and y",
/// );
/// ```
///
/// ## Additional examples
///
/// Additional examples can be found at
/// <https://tamaskis.github.io/plotting/plot_3d/quick_3d_plot_with_labels.html>.
pub fn quick_plot_3d_with_labels(
    x: impl Into<Vec<f64>>,
    y: impl Into<Vec<f64>>,
    z: impl Into<Vec<f64>>,
    x_label: impl Into<String>,
    y_label: impl Into<String>,
    z_label: impl Into<String>,
    title: impl Into<String>,
) -> Figure {
    // Create the trace.
    let trace = Trace::new_3d(x.into(), y.into(), z.into());

    // Formatting.
    let format = FormatBuilder::default()
        .x_label(x_label.into())
        .y_label(y_label.into())
        .z_label(z_label.into())
        .title(title.into())
        .build()
        .unwrap();

    // Create and return the figure.
    Figure::new(vec![trace], format)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_plot_2d() {
        let fig: Figure = quick_plot_2d([1.0, 2.0, 3.0], [1.0, 4.0, 9.0]);
        assert_eq!(fig.traces[0].x, [1.0, 2.0, 3.0]);
        assert_eq!(fig.traces[0].y, [1.0, 4.0, 9.0]);
        assert!(fig.format.title.is_none());
        assert!(fig.format.x_label.is_none());
        assert!(fig.format.y_label.is_none());
        assert!(fig.format.width.is_none());
        assert!(fig.format.height.is_none());
    }

    #[test]
    fn test_quick_plot_2d_with_labels() {
        let fig: Figure =
            quick_plot_2d_with_labels([1.0, 2.0, 3.0], [1.0, 4.0, 9.0], "x", "y", "y vs. x");
        assert_eq!(fig.traces[0].x, [1.0, 2.0, 3.0]);
        assert_eq!(fig.traces[0].y, [1.0, 4.0, 9.0]);
        assert_eq!(fig.format.title.unwrap(), "y vs. x");
        assert_eq!(fig.format.x_label.unwrap(), "x");
        assert_eq!(fig.format.y_label.unwrap(), "y");
        assert!(fig.format.width.is_none());
        assert!(fig.format.height.is_none());
    }

    #[test]
    fn test_quick_plot_3d() {
        let fig: Figure = quick_plot_3d([1.0, 2.0, 10.0], [1.0, 4.0, 9.0], [2.0, 5.0, 10.0]);
        assert_eq!(fig.traces[0].x, [1.0, 2.0, 10.0]);
        assert_eq!(fig.traces[0].y, [1.0, 4.0, 9.0]);
        assert_eq!(fig.traces[0].z.clone().unwrap(), [2.0, 5.0, 10.0]);
        assert!(fig.format.title.is_none());
        assert!(fig.format.x_label.is_none());
        assert!(fig.format.y_label.is_none());
        assert!(fig.format.z_label.is_none());
        assert!(fig.format.width.is_none());
        assert!(fig.format.height.is_none());
    }

    #[test]
    fn test_quick_plot_3d_with_labels() {
        let fig: Figure = quick_plot_3d_with_labels(
            [1.0, 2.0, 10.0],
            [1.0, 4.0, 9.0],
            [2.0, 5.0, 10.0],
            "x",
            "y",
            "z",
            "z vs. x and y",
        );
        assert_eq!(fig.traces[0].x, [1.0, 2.0, 10.0]);
        assert_eq!(fig.traces[0].y, [1.0, 4.0, 9.0]);
        assert_eq!(fig.traces[0].z.clone().unwrap(), [2.0, 5.0, 10.0]);
        assert_eq!(fig.format.title.unwrap(), "z vs. x and y");
        assert_eq!(fig.format.x_label.unwrap(), "x");
        assert_eq!(fig.format.y_label.unwrap(), "y");
        assert_eq!(fig.format.z_label.unwrap(), "z");
        assert!(fig.format.width.is_none());
        assert!(fig.format.height.is_none());
    }
}
