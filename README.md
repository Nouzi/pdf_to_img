Welcome! This is a simple, fast command-line tool written in Rust that converts the first page of a PDF document into a high-quality PNG image. 

I am currently in the first year of my Master's degree in Applied Informatics, and I am building this project in my spare time to teach myself Rust. My ultimate goal is to grow this into a "better," more intuitive alternative to existing PDF manipulation tools. Because this is a learning journey for me, this project is as much about building a welcoming community and learning from each other as it is about writing code!

## 📦 Getting Started & Binaries

To make things as easy as possible, you **do not** need to hunt down and install the underlying PDF rendering libraries yourself. 

I provide pre-compiled binaries for this tool, alongside the exact `pdfium` shared libraries (like `.so` for Linux/macOS and `.dll`/executables for Windows) it relies on, sourced directly from the `pdfium-render` project. 

**Why use the included binaries?**
The `pdfium` binaries included in this repository are the exact versions I use to develop and test this tool. By using the provided files, you are guaranteed full compatibility and stability, avoiding the headaches of mismatched versions or broken system installations. 

### How to Run It

**Option 1: Using the Pre-compiled Binaries (Easiest)**
1. Download the latest release from the repository (which includes the `pdf_to_img` converter binary and the required `pdfium` files).
2. Keep the `pdfium` files in the exact same directory as the converter executable.
3. Open your terminal or command prompt and run the tool depending on your operating system:
   **For Linux / macOS:**
   ```bash
   ./pdf_to_img input_document.pdf output_image.png
   ```
   **For Windows (Command Prompt or PowerShell):**
   ```powershell
   .\pdf_to_img.exe input_document.pdf output_image.png
   ```
**Option 2: Building from Source**
If you prefer to compile the Rust code yourself:
1. Ensure you have [Rust and Cargo installed](https://www.rust-lang.org/tools/install).
2. Clone the repository:
   ```bash
   git clone https://github.com/Nouzi/pdf_to_img.git
   cd pdf_to_img
   ```
3. Run the project (ensure the provided `pdfium` binaries are in your project root):
   ```bash
   cargo run -- input_document.pdf output_image.png
   ```
## ⚠️ Disclaimer & Current Limitations

This project is in its early stages. Here is what you need to know about the current version:
* **First Page Only:** Right now, the tool specifically targets and exports only the *first page* of the provided PDF.
* **Color Accuracy:** If you require 100% pixel-perfect, print-grade color replication, this tool might not be the best fit just yet. The underlying rendering technology currently handles standard conversions beautifully, but highly complex color profiles may not perfectly match the source. 

## 🛠️ How It Works (The Tech)

This tool relies on the `pdfium-render` crate. Here is a quick breakdown of what happens under the hood:
1. **The Engine:** We bind to **Pdfium**, which is the same open-source PDF rendering engine that powers Google Chrome.
2. **The Process:** The Rust code loads the PDF document into memory using Pdfium, targets the first page, and configures a renderer (currently set to a width of 2000 pixels for crisp quality). 
3. **The Output:** The rendered page is extracted as a bitmap and saved safely to your disk as a PNG using standard image processing.

## 🤝 Let's Collaborate!

I want this to be a project where people can learn, experiment, and help build something great. Since I am self-teaching Rust, **every opinion, code review, and piece of advice is incredibly valuable to me.**

Whether you are a Rust veteran who wants to point out best practices, or a complete beginner who wants to fix a typo in this README, your help is entirely welcome! 

**Ways you can help right now:**
* 🐛 Test it out and open an Issue if it breaks.
* 💡 Suggest features (like adding support for multi-page exports or custom resolutions).
* 🔧 Submit a Pull Request with code improvements or better error handling.

Let's build something awesome together!
