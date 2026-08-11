pub enum TaskAction {
    Amend { idx: usize, new: crate::Task },
    Create(crate::Task),
    Delete(usize),
    Save,
    ToggleDone(usize),
    None,
}
