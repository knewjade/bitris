/// Converting to different data with additional data.
pub trait With<T> {
    type Output;

    fn with(self, arg: T) -> Self::Output;
}
