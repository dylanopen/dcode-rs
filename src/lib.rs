pub mod console
{
	pub fn out<Text: Into<String>> (text: Text)
	{
		print!("{}", text.into());
	}

	pub fn outln<Text: Into<String>> (text: Text)
	{
		print!("{}\n", text.into());
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
