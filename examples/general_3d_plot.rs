use plotting::{Color, Format, FormatBuilder, LineStyle, NamedColor, Plot, Trace};
use std::path::Path;

fn main() {
    // Define the traces.
    let trace_1 = Trace::new_3d([1.0, 2.0, 5.0], [1.0, 2.0, 3.0], [1.0, 2.0, 4.0])
        .name("Trace 1")
        .line_color(Color::named(NamedColor::Red))
        .line_width(2.0)
        .line_style(LineStyle::Dash);
    let trace_2 = Trace::new_3d([1.0, 2.0, 5.0], [3.0, 2.0, 1.0], [1.0, 2.0, 4.0])
        .name("Trace 2")
        .line_color(Color::named(NamedColor::Blue))
        .line_width(2.0)
        .line_style(LineStyle::Dot);

    // Plot formatting.
    let format: Format = FormatBuilder::default()
        .title("z vs. x and y")
        .x_label("x")
        .y_label("y")
        .z_label("z")
        .width(800)
        .height(600)
        .build()
        .unwrap();

    // Create the plot.
    let plot = Plot::new(vec![trace_1, trace_2], format);

    // Save the plot so it can be displayed right below this example.
    plot.save_html(Path::new("book/src/figures/general_3d_plot.html"))
        .unwrap();

    // Alternatively, you can show the plot in a web browser.
    // plot.show();
}
