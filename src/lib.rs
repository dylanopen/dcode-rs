pub mod console
{
	pub fn out(text: String)
	{
		print!("{}", text);
	}
}





#[cfg(test)]
mod tests
{
	//use super::*;

	#[test]
	fn test_example()
	{
		assert_eq!(5, 5);
	}
}
