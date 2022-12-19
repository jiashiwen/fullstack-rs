use crate::casbin_rbatis_adapter::actions as adepter;
use axum::async_trait;
use casbin::{error::AdapterError, Adapter, Error as CasbinError, Filter, Model, Result};
use rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;

use super::{
    actions::{
        add_policies, add_policy, clear_policy, load_policy, remove_filtered_policy,
        remove_policies, remove_policy, save_policy,
    },
    tables::CasbinRule,
};

pub const TABLE_NAME: &str = "casbin_rule";

#[derive(Debug)]
pub struct CasbinRbatisAdapter {
    rbatis: rbatis::Rbatis,
    is_filtered: bool,
}

impl CasbinRbatisAdapter {
    pub async fn new<U: Into<String>>(url: U, pool_size: usize) -> Result<Self> {
        let rb = Rbatis::new();
        rb.init(MysqlDriver {}, &url.into())
            .map_err(|err| CasbinError::from(AdapterError(Box::new(rbatis::Error::from(err)))))?;
        let pool = rb
            .get_pool()
            .map_err(|err| CasbinError::from(AdapterError(Box::new(rbatis::Error::from(err)))))?;
        pool.resize(pool_size);
        adepter::new(&rb).await.map(|_| Self {
            rbatis: rb,
            is_filtered: false,
        })
    }
}

#[async_trait]
impl Adapter for CasbinRbatisAdapter {
    async fn load_policy(&self, m: &mut dyn Model) -> Result<()> {
        let mut rb = self.rbatis.clone();

        // #[cfg(feature = "runtime-tokio")]
        // let rules = spawn_blocking(move || action::load_policy(conn))
        //     .await
        //     .map_err(|e| casbin::error::AdapterError(Box::new(e)))??;

        // #[cfg(feature = "runtime-async-std")]
        // let rules = spawn_blocking(move || adapter::load_policy(conn)).await?;

        let rules = load_policy(&mut rb).await?;

        for casbin_rule in &rules {
            let rule = load_policy_line(casbin_rule);

            if let Some(ptype) = casbin_rule.ptype.clone() {
                if let Some(ref sec) = ptype.chars().next().map(|x| x.to_string()) {
                    if let Some(t1) = m.get_mut_model().get_mut(sec) {
                        if let Some(t2) = t1.get_mut(&ptype) {
                            if let Some(rule) = rule {
                                t2.get_mut_policy().insert(rule);
                            }
                        }
                    }
                }
            }

            // if let Some(ref sec) = casbin_rule.ptype.chars().next().map(|x| x.to_string()) {
            //     if let Some(t1) = m.get_mut_model().get_mut(sec) {
            //         if let Some(t2) = t1.get_mut(&casbin_rule.ptype) {
            //             if let Some(rule) = rule {
            //                 t2.get_mut_policy().insert(rule);
            //             }
            //         }
            //     }
            // }
        }
        Result::Ok(())
    }
    async fn load_filtered_policy<'a>(&mut self, m: &mut dyn Model, f: Filter<'a>) -> Result<()> {
        // #[cfg(feature = "runtime-tokio")]
        // let rules = spawn_blocking(move || adapter::load_policy(conn))
        //     .await
        //     .map_err(|e| casbin::error::AdapterError(Box::new(e)))??;

        // #[cfg(feature = "runtime-async-std")]
        // let rules = spawn_blocking(move || adapter::load_policy(conn)).await?;

        let rules = load_policy(&mut self.rbatis).await?;

        for casbin_rule in &rules {
            let rule = load_filtered_policy_line(casbin_rule, &f);

            if let Some((is_filtered, rule)) = rule {
                if is_filtered {
                    self.is_filtered = is_filtered;
                    if let Some(ptype) = casbin_rule.ptype.clone() {
                        if let Some(ref sec) = ptype.chars().next().map(|x| x.to_string()) {
                            if let Some(t1) = m.get_mut_model().get_mut(sec) {
                                if let Some(t2) = t1.get_mut(&ptype) {
                                    t2.get_mut_policy().insert(rule);
                                }
                            }
                        }
                    }
                }
            }
        }

        Result::Ok(())
    }
    async fn save_policy(&mut self, m: &mut dyn Model) -> Result<()> {
        // let conn = self
        //     .pool
        //     .get()
        //     .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::PoolError(err)))))?;

        let mut rules = vec![];

        if let Some(ast_map) = m.get_model().get("p") {
            for (ptype, ast) in ast_map {
                let new_rules = ast
                    .get_policy()
                    .into_iter()
                    .filter_map(|x: &Vec<String>| save_policy_line(ptype, x));

                rules.extend(new_rules);
            }
        }

        if let Some(ast_map) = m.get_model().get("g") {
            for (ptype, ast) in ast_map {
                let new_rules = ast
                    .get_policy()
                    .into_iter()
                    .filter_map(|x: &Vec<String>| save_policy_line(ptype, x));

                rules.extend(new_rules);
            }
        }

        // #[cfg(feature = "runtime-tokio")]
        // {
        //     spawn_blocking(move || adapter::save_policy(conn, rules))
        //         .await
        //         .map_err(|e| casbin::error::AdapterError(Box::new(e)))?
        // }
        // #[cfg(feature = "runtime-async-std")]
        // {
        //     spawn_blocking(move || adapter::save_policy(conn, rules)).await
        // }
        save_policy(&self.rbatis, rules).await?;
        Result::Ok(())
    }
    async fn clear_policy(&mut self) -> Result<()> {
        // let conn = self
        //     .pool
        //     .get()
        //     .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::PoolError(err)))))?;

        // #[cfg(feature = "runtime-tokio")]
        // {
        //     spawn_blocking(move || adapter::clear_policy(conn))
        //         .await
        //         .map_err(|e| casbin::error::AdapterError(Box::new(e)))?
        // }
        // #[cfg(feature = "runtime-async-std")]
        // {
        //     spawn_blocking(move || adapter::clear_policy(conn)).await
        // }

        clear_policy(&self.rbatis).await?;
        Result::Ok(())
    }
    fn is_filtered(&self) -> bool {
        false
    }
    async fn add_policy(&mut self, _sec: &str, ptype: &str, rule: Vec<String>) -> Result<bool> {
        // let conn = self
        //     .pool
        //     .get()
        //     .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::PoolError(err)))))?;
        let ptype_c = ptype.to_string();
        let mut rb = self.rbatis.clone();

        // #[cfg(feature = "runtime-tokio")]
        // {
        //     spawn_blocking(move || {
        //         if let Some(new_rule) = save_policy_line(&ptype_c, &rule) {
        //             return adapter::add_policy(conn, new_rule);
        //         }
        //         Ok(false)
        //     })
        //     .await
        //     .map_err(|e| casbin::error::AdapterError(Box::new(e)))?
        // }

        // #[cfg(feature = "runtime-async-std")]
        // {
        //     spawn_blocking(move || {
        //         if let Some(new_rule) = save_policy_line(&ptype_c, &rule) {
        //             return adapter::add_policy(conn, new_rule);
        //         }
        //         Ok(false)
        //     })
        //     .await
        // }
        if let Some(new_rule) = save_policy_line(&ptype_c, &rule) {
            return add_policy(&mut rb, new_rule).await;
        }

        Result::Ok(true)
    }

    async fn add_policies(
        &mut self,
        _sec: &str,
        ptype: &str,
        rules: Vec<Vec<String>>,
    ) -> Result<bool> {
        let new_rules = rules
            .iter()
            .filter_map(|x| save_policy_line(ptype, x))
            .collect::<Vec<CasbinRule>>();

        add_policies(&self.rbatis, new_rules).await
    }

    async fn remove_policy(&mut self, _sec: &str, ptype: &str, rule: Vec<String>) -> Result<bool> {
        remove_policy(&self.rbatis, ptype, rule).await
    }

    async fn remove_policies(
        &mut self,
        _sec: &str,
        ptype: &str,
        rules: Vec<Vec<String>>,
    ) -> Result<bool> {
        remove_policies(&self.rbatis, ptype, rules).await
    }
    async fn remove_filtered_policy(
        &mut self,
        _sec: &str,
        ptype: &str,
        field_index: usize,
        field_values: Vec<String>,
    ) -> Result<bool> {
        if field_index <= 5 && !field_values.is_empty() && field_values.len() > field_index {
            remove_filtered_policy(&self.rbatis, ptype, field_index, field_values).await
        } else {
            Ok(false)
        }
    }
}

pub(crate) fn save_policy_line(ptype: &str, rule: &[String]) -> Option<CasbinRule> {
    if ptype.trim().is_empty() || rule.is_empty() {
        return None;
    }

    let mut new_rule = CasbinRule {
        id: None,
        ptype: Some(ptype.to_owned()),
        v0: Some("".to_owned()),
        v1: Some("".to_owned()),
        v2: Some("".to_owned()),
        v3: Some("".to_owned()),
        v4: Some("".to_owned()),
        v5: Some("".to_owned()),
    };

    new_rule.v0 = Some(rule[0].to_owned());

    if rule.len() > 1 {
        new_rule.v1 = Some(rule[1].to_owned());
    }

    if rule.len() > 2 {
        new_rule.v2 = Some(rule[2].to_owned());
    }

    if rule.len() > 3 {
        new_rule.v3 = Some(rule[3].to_owned());
    }

    if rule.len() > 4 {
        new_rule.v4 = Some(rule[4].to_owned());
    }

    if rule.len() > 5 {
        new_rule.v5 = Some(rule[5].to_owned());
    }

    Some(new_rule)
}

pub(crate) fn load_policy_line(casbin_rule: &CasbinRule) -> Option<Vec<String>> {
    if let Some(ptype) = casbin_rule.ptype.clone() {
        if ptype.chars().next().is_some() {
            return normalize_policy(casbin_rule);
        }
    }
    None
}

pub(crate) fn load_filtered_policy_line(
    casbin_rule: &CasbinRule,
    f: &Filter,
) -> Option<(bool, Vec<String>)> {
    if let Some(ptype) = &casbin_rule.ptype {
        if let Some(sec) = ptype.chars().next() {
            if let Some(policy) = normalize_policy(casbin_rule) {
                let mut is_filtered = true;
                if sec == 'p' {
                    for (i, rule) in f.p.iter().enumerate() {
                        if !rule.is_empty() && rule != &policy[i] {
                            is_filtered = false
                        }
                    }
                } else if sec == 'g' {
                    for (i, rule) in f.g.iter().enumerate() {
                        if !rule.is_empty() && rule != &policy[i] {
                            is_filtered = false
                        }
                    }
                } else {
                    return None;
                }
                return Some((is_filtered, policy));
            }
        }
    }
    None
}

fn normalize_policy(casbin_rule: &CasbinRule) -> Option<Vec<String>> {
    let mut result = vec![
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
    ];

    if let Some(v0) = casbin_rule.v0.clone() {
        result[0] = v0;
    }
    if let Some(v1) = casbin_rule.v1.clone() {
        result[1] = v1;
    }
    if let Some(v2) = casbin_rule.v2.clone() {
        result[2] = v2;
    }
    if let Some(v3) = casbin_rule.v3.clone() {
        result[3] = v3
    }
    if let Some(v4) = casbin_rule.v4.clone() {
        result[4] = v4
    }
    if let Some(v5) = casbin_rule.v5.clone() {
        result[5] = v5
    }

    while let Some(last) = result.last() {
        if last.is_empty() {
            result.pop();
        } else {
            break;
        }
    }

    if !result.is_empty() {
        return Some(result.iter().map(|x| x.to_owned()).collect());
    }

    None
}

#[cfg(test)]
mod test {
    use core::time;
    use std::thread;

    use crate::casbin_rbatis_adapter::{
        actions::{add_policy, remove_policy},
        adapter::CasbinRbatisAdapter,
        tables::CasbinRule,
    };

    //cargo test casbin_rbatis_adapter::adapter::test::test_casbin_rbatis_adapter --  --nocapture
    #[test]
    fn test_casbin_rbatis_adapter() {
        println!("test_casbin_rbatis_adapter");
        let url= "mysql://rust_test:Git785230root@mysql-internet-cn-north-1-1221449f8fb94332.rds.jdcloud.com:3306/rust_test";
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            let mut rule = CasbinRule {
                id: None,
                ptype: Some("p".to_string()),
                v0: Some("bob".to_string()),
                v1: Some("data".to_string()),
                v2: Some("read".to_string()),
                v3: Some("".to_string()),
                v4: Some("".to_string()),
                v5: Some("".to_string()),
            };
            let mut cra = CasbinRbatisAdapter::new(url, 3).await.unwrap();
            println!("casbin adapter is {:?}", cra);
            {
                let rs = add_policy(&mut cra.rbatis, rule.clone()).await;
                println!("add result: {:?}", rs);
            }
            {
                rule.v0 = Some("Jesica".to_string());
                let _ = add_policy(&mut cra.rbatis, rule.clone()).await;
            }
            thread::sleep(time::Duration::from_secs(4));
            let remove_rs = remove_policy(
                &mut cra.rbatis,
                "p",
                vec!["bob".to_string(), "data".to_string(), "read".to_string()],
            )
            .await;
            println!("remove result is {:?}", remove_rs);
            // let select_rs = cra
            //     .rbatis
            //     .clone()
            //     .fetch_decode::<CasbinRule>(
            //         "select * from casbin_rule where ptype = ? and v0 = ?;",
            //         // &sql_statment,
            //         vec![
            //             to_value!("p".to_string()),
            //             to_value!("bob".to_string()),
            //             // to_value!(normal_rule[1].clone()),
            //             // to_value!(normal_rule[2].clone()),
            //             // to_value!(normal_rule[3].clone()),
            //             // to_value!(normal_rule[4].clone()),
            //         ],
            //     )
            //     .await;
            // println!("select result is {:?}", select_rs);
        });
    }
}
