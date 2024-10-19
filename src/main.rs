use eframe::egui;

fn main() {
    let app = SpreadsheetApp::new(20, 20); // Initialize with 20x20 grid
    eframe::run_native(
        "Spreadsheet App",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(app))),
    );
}

#[derive(Debug)]
struct SpreadsheetApp {
    data: Vec<Vec<String>>, // 2D matrix of cell data
    rows: usize,
    cols: usize,
}

impl SpreadsheetApp {
    fn new(rows: usize, cols: usize) -> Self {
        // Initialize the data grid with empty strings
        SpreadsheetApp {
            data: vec![vec!["".to_string(); cols]; rows],
            rows,
            cols,
        }
    }
}

impl eframe::App for SpreadsheetApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Spreadsheet App");

            // Wrap the grid in a scroll area
            egui::ScrollArea::both()
                .auto_shrink([false; 2]) // Disable auto-shrinking to ensure the grid stays in view
                .show(ui, |ui| {
                    egui::Grid::new("spreadsheet_grid")
                        .striped(true)
                        .num_columns(self.cols + 1) // One extra column for row headers
                        .show(ui, |ui| {
                            // First row: Empty cell + Column headers (A, B, C, ...)
                            ui.label(""); // Top-left empty cell
                            for col in 0..self.cols {
                                ui.label(format!("{}", (b'A' + col as u8) as char)); // A, B, C, ...
                            }
                            ui.end_row();

                            // Render the rest of the grid with row headers and data cells
                            for row in 0..self.rows {
                                // Row header (1, 2, 3, ...)
                                ui.label(format!("{}", row + 1));

                                // Render data cells for this row
                                for col in 0..self.cols {
                                    let row_data = self.data.get_mut(row).unwrap();
                                    let cell = row_data.get_mut(col).unwrap(); // Get reference to the cell data
                                    let mut input = cell.clone(); // Clone the cell value for editing

                                    ui.text_edit_singleline(&mut input); // Edit the cell

                                    // If the input has changed, update the cell in the data vector
                                    if input != *cell {
                                        *cell = input; // Save updated input to the cell
                                    }
                                }
                                ui.end_row();
                            }
                        });
                });
        });
    }
}

