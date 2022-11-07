use crate::httpserver::module::User;
use crate::privilege::{create_user, get_user_by_name};
use crate::privilege::{remove_user, User as PrivilegeUser};
use anyhow::Result;

pub fn s_user_create(u: User) -> Result<()> {
    create_user(u.username, u.password)
}

pub fn s_remove_user(id: String) -> Result<()> {
    remove_user(id)
}

pub fn s_get_user(name: String) -> Result<PrivilegeUser> {
    get_user_by_name(name)
}
