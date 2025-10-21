# Rusty_SVG2ICO

A simple desktop GUI application built with Rust and egui for converting SVG files to multi-resolution ICO (Icon) files that are Windows Compatible.
It also allows viewing existing ICO files.

## Features

- **Convert SVG to ICO**: Select an SVG file and generate an ICO with multiple sizes (256x256, 128x128, 64x64, 48x48, 32x32, 24x24, 16x16).
- **View ICO Files**: Load and display existing ICO files with all their embedded sizes.
- **Save Generated ICO**: Save the converted ICO to a file on disk.
- **User-Friendly Interface**: Clean GUI with buttons for file selection and a scrollable display area for icons.

## Requirements

- Rust (latest stable version recommended)
- Windows (built with eframe/egui for native Windows GUI)

## Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd WIN_SVG2ICO
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the application:
   ```bash
   cargo run --release
   ```

## Usage

1. Launch the application.
2. Click "Select SVG File" to choose an SVG file. The app will convert it to ICO and display all sizes.
3. Alternatively, click "Open ICO File" to load an existing ICO for viewing.
4. If an ICO was generated, use "Save Icon" to save it to disk.
5. The display box shows each icon size with its resolution, scrollable if needed.

## Dependencies

- `eframe` & `egui` & `egui_extras`: For the GUI framework.
- `rfd`: For file dialogs.
- `svg_to_ico`: For SVG to ICO conversion.
- `image` & `ico`: For image processing and ICO parsing.
- `tempfile`: For temporary file handling.

## Contributing

Feel free to submit issues or pull requests for improvements.

## License

This project is open-source. See LICENSE file if present.