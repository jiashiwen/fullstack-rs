use crate::errors::{GlobalError, GlobalErrorType};
use anyhow::{anyhow, Result};
use base64;
use chrono::Local;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

use dashmap::DashMap;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Clone, Debug, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub password: String,
}

impl User {
    pub fn default() -> Self {
        let id = base64::encode("0");
        Self {
            id,
            name: "".to_string(),
            password: "".to_string(),
        }
    }

    pub fn new(name: String, password: String) -> Self {
        let id = base64::encode(name.clone());
        Self { id, name, password }
    }

    pub fn change_password(&mut self, password: String) {
        self.password = password;
    }
}

lazy_static::lazy_static! {
    // 初始化GLOBAL_USER_MAP的key
    static ref USER_LATEST: String=String::from("latest");
    // 全局用户映射 user_id:User
    static ref GLOBAL_USER_MAP: RwLock<HashMap<String,User>> = RwLock::new({
        let mut map = HashMap::new();
        let id = base64::encode("root".to_string());
        let root=User::new("root".to_string(),"123456".to_string());
        map.insert(id,root);
        map
    });
    // 全局token映射，token:userid
  static ref GLOBAL_TOKEN_MAP: DashMap<String, String>  =  {
        let map = DashMap::new();
        map.insert("".to_string(),"".to_string());
        map
    };

    // 全局登录状态，userid:token
      static ref GLOBAL_LOGIN_STATUS: DashMap<String,String>  =  {
        let  map=DashMap::new();
        map.insert("".to_string(),"".to_string());
        map
    };

}

// 生成token
// 检查GLOBAL_LOGIN_STATUS 中是否已有token生成，若有直接返回
// 若没有，生成token存入GLOBAL_LOGIN_STATUS和GLOBAL_TOKEN_MAP
pub fn gen_token(user: User) -> Result<String> {
    let id = user.id.clone();
    let token = GLOBAL_LOGIN_STATUS.get(&id);

    return match token {
        None => {
            let dt = Local::now();
            let seed = format!("{}{}{}", user.name, user.password, dt.timestamp_millis());
            let mut hasher = Sha256::new();
            hasher.input_str(seed.as_str());
            let token = hasher.result_str();
            token_store(id.clone(), token.clone())?;
            Ok(token)
        }
        Some(t) => {
            let r = t.value().clone();
            Ok(r)
        }
    };
}

pub fn get_user_id_from_token(token: String) -> Result<String> {
    let id = GLOBAL_TOKEN_MAP.get(&token);
    match id {
        None => Err(anyhow!("token not exists")),
        Some(id) => Ok(id.value().clone()),
    }
}

pub fn token_store(userid: String, token: String) -> Result<()> {
    GLOBAL_LOGIN_STATUS.insert(userid.clone(), token.clone());
    GLOBAL_TOKEN_MAP.insert(token.clone(), userid.clone());
    Ok(())
}

pub fn token_remove(token: String) -> Result<()> {
    let t = token.clone();
    let id = {
        let opt = GLOBAL_TOKEN_MAP.get(token.as_str());
        match opt {
            None => Err(anyhow!("")),
            Some(pair) => Ok(pair.value().to_string().clone()),
        }
    };

    return match id {
        Ok(idstr) => {
            GLOBAL_TOKEN_MAP.remove(&*t);
            GLOBAL_LOGIN_STATUS.remove(&*idstr);
            Ok(())
        }
        Err(e) => Err(e),
    };
}

pub fn create_user(name: String, password: String) -> Result<()> {
    let exist = user_exist(name.clone())?;
    if exist {
        return Err(anyhow!("user exists"));
    }
    let id = base64::encode(name.clone());
    let mut wmap = GLOBAL_USER_MAP.write().map_err(|e| {
        return GlobalError::from_err(e.to_string(), GlobalErrorType::UnknowErr);
    })?;
    wmap.insert(id.clone(), User { id, name, password });
    Ok(())
}

pub fn user_exist(name: String) -> Result<bool> {
    let id = base64::encode(name.clone());
    let rl_map = GLOBAL_USER_MAP.read().map_err(|e| {
        return GlobalError::from_err(e.to_string(), GlobalErrorType::UnknowErr);
    })?;
    let rs = rl_map.get(&id);

    if let None = rs {
        return Ok(false);
    }
    Ok(true)
}

pub fn get_user_by_name(name: String) -> Result<User> {
    let rl_map = GLOBAL_USER_MAP.read().map_err(|e| {
        return GlobalError::from_err(e.to_string(), GlobalErrorType::UnknowErr);
    })?;
    let id = base64::encode(name.clone());
    let user = rl_map.get(&id);
    match user {
        None => {
            return Err(anyhow!("user not exists"));
        }
        Some(u) => Ok(u.clone()),
    }
}

pub fn get_user_by_id(id: String) -> Result<User> {
    let rl_map = GLOBAL_USER_MAP.read().map_err(|e| {
        return GlobalError::from_err(e.to_string(), GlobalErrorType::UnknowErr);
    })?;
    let user = rl_map.get(&id);
    match user {
        None => {
            return Err(anyhow!("user not exists"));
        }
        Some(u) => Ok(u.clone()),
    }
}

pub fn remove_user(id: String) -> Result<()> {
    let mut wr_map = GLOBAL_USER_MAP.write().map_err(|e| {
        return GlobalError::from_err(e.to_string(), GlobalErrorType::UnknowErr);
    })?;
    wr_map.remove(&id);
    Ok(())
}
