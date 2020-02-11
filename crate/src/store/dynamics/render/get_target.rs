#[derive(Debug, PartialEq)]
pub enum Web {
    Context2D,
    WebGL,
}

#[derive(Debug, PartialEq)]
pub enum Target {
    Web(Web),
}
