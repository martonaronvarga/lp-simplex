#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VarKind {
    Original(usize),
    Artificial(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    pub kind: VarKind,
    pub name: String,
}

impl Variable {
    // pub fn is_original(&self) -> bool {
    //     matches!(self.kind, VarKind::Original(_))
    // }
    pub fn is_artificial(&self) -> bool {
        matches!(self.kind, VarKind::Artificial(_))
    }
}
