use u_u::jpeg_to_svg;

const JPEG_BYTES: &[u8] = include_bytes!("./u_u.jpg");

fn main() {
    let svg_bytes = jpeg_to_svg(JPEG_BYTES).unwrap();
    std::fs::write("./output_example.svg", svg_bytes).unwrap();
}
