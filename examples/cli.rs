use std::path::Path;
use u_u::jpeg_to_svg;

fn main() {
    let args = std::env::args().into_iter().collect::<Vec<String>>();
    if args.len() != 3 {
        eprintln!("usage: input.jpeg output.svg");
        std::process::exit(1);
    }
    let current_path = std::env::current_dir().unwrap().into_boxed_path();
    let in_file = Path::new(&args[1]);
    let out_file = Path::new(&args[2]);
    let in_path = Path::join(&current_path, &in_file);
    let out_path = Path::join(&current_path, &out_file);
    let in_bytes = std::fs::read(in_path).unwrap();
    let bytes_ref: &[u8] = &in_bytes;
    let svg_bytes = jpeg_to_svg(bytes_ref).unwrap();
    std::fs::write(out_path, svg_bytes).unwrap();
}
