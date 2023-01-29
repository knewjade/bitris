/// Converting to different data with additional data.
pub trait With<T> {
    type Output;

    fn with(self, arg: T) -> Self::Output;
}

/// A collection of statements that instruct generation to continue/stop.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub enum GenerateInstruction {
    #[default] Continue,
    Stop,
}
