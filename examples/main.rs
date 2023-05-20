use u_u::jpeg_to_svg;

const JPEG_BYTES: &[u8] = include_bytes!("./diagram.jpg");

fn main() {
    jpeg_to_svg(JPEG_BYTES).unwrap();
}
