pub trait Decouple {
    type Error;
    fn parse() -> Result<Self, Self::Error>
    where
        Self: Sized;
}
