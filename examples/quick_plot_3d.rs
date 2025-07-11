use plotting::{Plot, quick_plot_3d};
use std::path::Path;

fn main() {
    // Create the plot.
    let plot: Plot = quick_plot_3d([1.0, 2.0, 10.0], [1.0, 4.0, 9.0], [2.0, 5.0, 10.0]);

    // Save the plot so it can be displayed right below this example.
    plot.save_inline_html(Path::new("book/src/figures/quick_plot_3d.html"))
        .unwrap();

    // Alternatively, you can show the plot in a web browser.
    // plot.show();
}
