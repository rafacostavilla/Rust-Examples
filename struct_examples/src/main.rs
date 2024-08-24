struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main(){
    let mut user1 = User{
        active: true,
        email: String::from("rafaelcostavilla@gmail.com"),
        username: String::from("rafaelcostavilla"),
        sign_in_count: 0,
    };
    let email = String::from("test@test.com");
    user1.active =  false;
    let _email2 = &email;
}