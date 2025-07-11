use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // Define the path to the `examples/` directory.
    let examples_dir = Path::new("examples");

    // Attempt to read the contents of the `examples/` directory.
    let entries = match fs::read_dir(examples_dir) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Failed to read examples directory: {err}");
            std::process::exit(1);
        }
    };

    // Iterate over all `.rs` files in the examples directory.
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();

        // Only consider files with a `.rs` extension.
        if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            // Extract the file stem (e.g., `foo.rs` → `foo`).
            if let Some(filename) = path.file_stem().and_then(|f| f.to_str()) {
                // Skip this file to prevent infinite recursion.
                if filename == "run_all" {
                    continue;
                }

                // Print the example being executed.
                println!("Running example: {filename}");

                // Run `cargo run --example <filename>` for each example.
                let status = Command::new("cargo")
                    .args(["run", "--example", filename])
                    .status()
                    .unwrap_or_else(|e| {
                        eprintln!("Failed to run example '{filename}': {e}");
                        std::process::exit(1);
                    });

                // Print an error if the example did not exit successfully.
                if !status.success() {
                    eprintln!("Example '{filename}' failed with status: {status}");
                }
            }
        }
    }
}
