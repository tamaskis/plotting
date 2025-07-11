use plotting::{Plot, quick_plot_2d};
use std::path::Path;

fn main() {
    // Create the plot.
    let plot: Plot = quick_plot_2d([1.0, 2.0, 3.0], [1.0, 4.0, 9.0]);

    // Save the plot so it can be displayed right below this example.
    plot.save_html(Path::new("book/src/figures/quick_plot_2d.html"))
        .unwrap();

    // Alternatively, you can show the plot in a web browser.
    // plot.show();
}
