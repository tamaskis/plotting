use plotting::{Figure, quick_plot_3d};
use std::path::Path;

fn main() {
    // Create the figure.
    let fig: Figure = quick_plot_3d([1.0, 2.0, 10.0], [1.0, 4.0, 9.0], [2.0, 5.0, 10.0]);

    // Save the figure so it can be displayed right below this example.
    fig.save_inline_html(Path::new("book/src/figures/quick_plot_3d.html"))
        .unwrap();

    // Alternatively, you can show the figure in a web browser.
    // fig.show();
}
