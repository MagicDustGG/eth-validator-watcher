#[cfg(test)]
mod addition {
	use crate::addition;

	#[test]
	fn works() {
		let result = addition(2, 2);
		assert_eq!(result, 4);
	}
}

#[cfg(test)]
mod substraction {
	use crate::substraction;

	#[test]
	fn works() {
		let result = substraction(5, 2);
		assert_eq!(result, 3);
	}
}
