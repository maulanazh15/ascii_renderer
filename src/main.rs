use clap::Parser;

mod ascii_renderer;

#[derive(Parser)]
struct Args {
    // Path file gambar
    file: String,
    // command untuk lebar gambar yang baru pada output ascii
    #[arg(short, long, default_value_t = 100)]
    width: u32,
    #[arg(short, long, default_value_t = false)]
    color: bool,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = ascii_renderer::render_ascii(&args) {
        eprintln!("Error : {}", e);
    }
}
