#[derive(Eq, PartialEq)]
pub struct Selector {
    pub s: &'static str,
    pub relative: bool
}

impl Selector {
    pub const fn direct(s: &'static str) -> Self {
        Self {
            s,
            relative: false,
        }
    }

    pub const fn relative(s: &'static str) -> Self {
        Self {
            s,
            relative: true,
        }
    }
}