#[derive(Debug, Clone, Copy)]
pub enum Operations {
    Add,
    Sub,
    Mul,
    Div,
}

impl std::fmt::Display for Operations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operations::Add => write!(f, "+"),
            Operations::Sub => write!(f, "-"),
            Operations::Mul => write!(f, "*"),
            Operations::Div => write!(f, "/"),
        }
    }
}
