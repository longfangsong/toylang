#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PhiSource {
    name: String,
    block: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Phi {
    to: String,
    from1: PhiSource,
    from2: PhiSource,
}
