fn main() {
    let user1 = User{
        email : String::from("someone@example.com"),
        username : String::from("Example"),
        active : true,
        sign_in_count : 1,
    };
    //how to modify values
    user1.email = String::from("test@example.com");
    //clunky way
    let user2 = User{
        active : user1.active,
        username : user1.username,
        email : String::from("another@example.com"),
        sign_in_count : user1.sign_in_count,
    };
    //simple way
    let user3 = User{
        email: String::from("Test@example.com"),
        ..user1
    };
    let black = Color(0,0,0);
}
struct User{
    active : bool,
    username : String,
    email : String,
    sign_in_count : u64,
}
fn build_user(email : String, username: String) -> User{
    User{
        email,
        username,
        active : true,
        sign_in_count : 1,
    }
}
//simple structs
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);