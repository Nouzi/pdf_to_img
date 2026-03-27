use pdfium_render::prelude::*;
use std::env;
use std::process::exit;

fn export_pdf_to_png(input_pdf_path: &str, output_image_path: &str) {
  let bind = Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
    .or_else(|_| Pdfium::bind_to_system_library())
    .expect("Failed to find pdfium shared library (libpdfium.so)");

  let pdfium = Pdfium::new(bind);

  let document =  match pdfium.load_pdf_from_file(input_pdf_path, None) {
    Ok(document) => document,
    Err(error) => {
      eprintln!("Failed to load pdf: {:?}", error);
      exit(1);
    }
  };

  let first_page = match document.pages().get(0) {
    Ok(page) => page,
    Err(e) => {
      eprintln!("Failed to load first page: {:?}", e);
      exit(1);
    }
  };

  let render_config = PdfRenderConfig::new().set_target_width(2000);
  let bitmap = match first_page.render_with_config(&render_config) {
    Ok(bitmap) => bitmap,
    Err(e) => {
      eprintln!("Failed to render page: {:?}", e);
      exit(1);
    }
  };

  match bitmap.as_image().save(output_image_path) {
    Ok(_) => println!("Image saved successfully to {}", output_image_path),
    Err(e) => {
      eprintln!("Failed to save image: {:?}", e);
      exit(1);
    },
  }

}

fn main() {
    println!("Starting PDF converter...");
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_pdf_path> <output_image_path>", args[0]);
        exit(1);
    }

    let input_pdf_path = &args[1];
    let output_image_path = &args[2];

    export_pdf_to_png(input_pdf_path, output_image_path);
}
