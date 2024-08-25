struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//Unit-Like Structs without any fields
//We can still use it without data to implement a behaviour
struct AlwaysEqual;

fn build_user (email: String, username: String) -> User{
    User { 
        active: true,
        username,
        email,
        sign_in_count: 1 
    }
}

fn main(){
    let mut user1 = User{
        active: true,
        email: String::from("rafaelcostavilla@gmail.com"),
        username: String::from("rafaelcostavilla"),
        sign_in_count: 1,
    };
    let email = String::from("test@test.com");
    user1.active =  false;
    user1.email = email;

    let user2 = build_user(String::from("outro@outro.com"), String::from("outro"));
    println!("{}", user2.email);

    let _user3 = User{
        email: String::from("another@example.com"),
        ..user1
    };
    //We cannot use the username from user1 because it has moved when we created user3 from the user1's username
    // println!("{}", user1.username);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}", black.0);
    println!("{}", origin.0);

    //We don't need parentheses or curly braces
    let subject = AlwaysEqual;
}