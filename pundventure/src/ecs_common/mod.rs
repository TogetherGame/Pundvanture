mod fonts;

pub(crate) mod components {
    pub(crate) use super::fonts::Fonts;
}

pub(crate) mod systems {
    pub(crate) use super::fonts::load_fonts;
}
