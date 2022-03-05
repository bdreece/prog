pub trait TryParseFrom<T> {
	type Error;
    fn try_parse_from(value: T) -> Result<Self, Self::Error>
        where Self: Sized;
}
