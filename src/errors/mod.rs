#[derive(Debug)]
pub enum ShaderCompile {
    Failed(String),
    Success,
}

#[derive(Debug)]
pub enum ShaderLink {
    Failed(String),
    Success,
}

#[derive(Clone, Debug)]
pub enum DrawStatus {
    Failed(String),
    Success
}