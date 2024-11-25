pub struct LoginDto {
    pub username: String,
    pub password: String,
}

pub fn login(dto: LoginDto) {
    println!("Login attempt for user: {}", dto.username);
}
