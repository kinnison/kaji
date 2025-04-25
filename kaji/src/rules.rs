use crate::PuzzleBuilder;

pub trait Rule {
    fn apply(&self, builder: &mut PuzzleBuilder);
}
