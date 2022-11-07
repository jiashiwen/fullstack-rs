#[cfg(test)]
mod user_test {
    use crate::privilege::user::{create_user, get_user_by_name, remove_user};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn create_user_test() {
        let rs = create_user("username".to_string(), "password".to_string());
        println!("rs is {:?}", rs);
        let user = get_user_by_name("username".to_string()).unwrap();
        println!("{:?}", user);

        remove_user(user.id);
        let af = get_user_by_name("username".to_string());

        match af {
            Ok(u) => {
                assert_eq!("username", u.name);
            }
            Err(e) => {
                assert_eq!("user not exists", e.to_string());
            }
        }
    }
}
