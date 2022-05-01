fn main() {
  let s1 = String::from("hello");
	let s2 = s1;
	// s1 is freed!!! trying to access s1 will give errors
	// We say s1 was moved to s2
	let s3 = s2.clone(); // deep copy
	// s2, s3 both exist
}