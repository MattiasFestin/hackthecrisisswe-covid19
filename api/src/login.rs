#[get("/login")]
fn login() -> &'static str {
	// "Hello, world!"
	if envmnt::exists("SCRYPT_SALT") {
		let SCRYPT_SALT = envmnt::get_or_panic("SCRYPT_SALT");
		let mut result = vec![0u8; t.expected.len()];
		let params = ScryptParams::new(14, 8, 1).unwrap();
		
		scrypt(t.password.as_bytes(), SCRYPT_SALT.as_bytes(), &params, &mut result)
			.unwrap();
	}
}

#[get("/logout")]
fn logout() -> &'static str {
    "Hello, world!"
}