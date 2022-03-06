use std::vec::Vec;

pub trait TryParseFrom<T> {
	type Error;
    fn try_parse_from(value: T) -> Result<Self, Self::Error>
        where Self: Sized;
}

pub trait Process {
	fn process(&mut self, args: &Vec<String>);
}
