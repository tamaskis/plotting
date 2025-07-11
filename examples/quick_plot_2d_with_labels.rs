use plotting::{Plot, quick_plot_2d_with_labels};
use std::path::Path;

fn main() {
    // Create the plot.
    let plot: Plot =
        quick_plot_2d_with_labels([1.0, 2.0, 3.0], [1.0, 4.0, 9.0], "x", "y", "y = x^2");

    // Save the plot so it can be displayed right below this example.
    plot.save_html(Path::new("book/src/figures/quick_plot_2d_with_labels.html"))
        .unwrap();

    // Alternatively, you can show the plot in a web browser.
    // plot.show();
}
