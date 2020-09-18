#[derive(Debug)]
pub struct PaperProps<'a> {
    pub name: &'a str,
    pub width: f32,
    pub height: f32,
}

pub static A4 : PaperProps = PaperProps { name: "A4", width: 595.0, height: 842.0 };