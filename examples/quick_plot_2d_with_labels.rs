use plotting::{Figure, quick_plot_2d_with_labels};
use std::path::Path;

fn main() {
    // Create the figure.
    let fig: Figure =
        quick_plot_2d_with_labels([1.0, 2.0, 3.0], [1.0, 4.0, 9.0], "x", "y", "y = x^2");

    // Save the figure so it can be displayed right below this example.
    fig.save_inline_html(Path::new("book/src/figures/quick_plot_2d_with_labels.html"));

    // Alternatively, you can show the figure in a web browser.
    // fig.show();
}
