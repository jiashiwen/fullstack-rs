use crate::privilege::authentication::{
    add_grouping_policy, remove_grouping_policy, remove_policy,
};
use crate::privilege::{add_policy, casbin_enforce};
use casbin::Result;

pub enum ObjType {
    User,
}

impl ObjType {
    pub fn to_string(&self) -> String {
        match self {
            ObjType::User => String::from("user"),
        }
    }
}

pub enum ActionType {
    Create,
    Remove,
    Update,
    Select,
}

impl ActionType {
    pub fn to_string(&self) -> String {
        match self {
            ActionType::Create => String::from("create"),
            ActionType::Remove => String::from("remove"),
            ActionType::Update => String::from("update"),
            ActionType::Select => String::from("select"),
        }
    }
}

pub struct Policy {
    sub: String,
    dom: String,
    obj: ObjType,
    act: ActionType,
}

impl Policy {
    pub fn set_sub(&mut self, sub: String) {
        self.sub = sub;
    }
}

impl Policy {
    pub fn new(sub: String, dom: String, obj: ObjType, act: ActionType) -> Self {
        Self { sub, dom, obj, act }
    }

    fn to_vec(&self) -> Vec<String> {
        let mut v = vec![];
        v.push(self.sub.clone());
        v.push(self.dom.clone());
        v.push(self.obj.to_string());
        v.push(self.act.to_string());
        v
    }

    pub async fn add(&self) -> Result<bool> {
        let v = self.to_vec();
        add_policy(v).await
    }

    pub async fn remove(&self) -> Result<bool> {
        let v = self.to_vec();
        remove_policy(v).await
    }

    pub async fn enforce(&self) -> Result<bool> {
        let v = self.to_vec();
        casbin_enforce(v).await
    }
}

pub struct GroupPolicy {
    sub: String,
    policy_sub: String,
    dom: String,
}

impl GroupPolicy {
    pub fn new(sub: String, policy_sub: String, dom: String) -> Self {
        Self {
            sub,
            policy_sub,
            dom,
        }
    }
    fn to_vec(&self) -> Vec<String> {
        let mut v = vec![];
        v.push(self.sub.clone());
        v.push(self.policy_sub.clone());
        v.push(self.dom.clone());
        v
    }

    pub async fn add(&self) -> Result<bool> {
        let v = self.to_vec();
        add_grouping_policy(v).await
    }

    pub async fn remove(&self) -> Result<bool> {
        let v = self.to_vec();
        remove_grouping_policy(v).await
    }
}
