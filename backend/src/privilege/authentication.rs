use crate::resources::{get_rbac_model, get_rbac_policy};
use async_once::AsyncOnce;
use casbin::prelude::*;
use casbin::MemoryAdapter;
use casbin::Result;
use futures_locks::RwLock;

use once_cell::sync::Lazy;

// lazy_static::lazy_static! {
//     static ref GLOBALE_CASBIN_ENFORCER: AsyncOnce<RwLock<CasbinEnforcer>> = AsyncOnce::new(async {
//         let ce=CasbinEnforcer::default().await;
//         RwLock::new(ce)
//     });
// }

// static GLOBALE_CASBIN_ENFORCER: Lazy<RwLock<CasbinEnforcer>> = Lazy::new(|| {
//     AsyncOnce::new(async {
//         let ce = CasbinEnforcer::default().await;
//         RwLock::new(ce)
//     });
// });

static GLOBALE_CASBIN_ENFORCER: Lazy<RwLock<CasbinEnforcer>> = Lazy::new(|| {
    let rw_casbin_enforcer = futures::executor::block_on(async {
        let ce = CasbinEnforcer::default().await;
        RwLock::new(ce)
    });
    rw_casbin_enforcer
});

pub struct CasbinEnforcer {
    enforcer: Enforcer,
}

impl CasbinEnforcer {
    pub async fn default() -> Self {
        let rbac_model = get_rbac_model().unwrap();
        let f = std::str::from_utf8(rbac_model.data.as_ref()).unwrap();
        let m = DefaultModel::from_str(f).await.unwrap();
        let a = MemoryAdapter::default();
        let mut enforcer = Enforcer::new(m, a).await.unwrap();

        let rbac_policy = get_rbac_policy().unwrap();
        let policy = std::str::from_utf8(rbac_policy.data.as_ref()).unwrap();
        let mut lines = policy.lines();
        loop {
            let line = lines.next();
            match line {
                None => {
                    break;
                }
                Some(l) => {
                    let mut ispolicy = true;
                    let mut w = l.split(',');
                    let mut v = vec![];
                    loop {
                        let iterm = w.next();
                        match iterm {
                            None => break,
                            Some(i) => {
                                if i.eq("p") {
                                    ispolicy = true;
                                    continue;
                                }
                                if i.eq("g") {
                                    ispolicy = false;
                                    continue;
                                }
                                v.push(i.to_string());
                            }
                        }
                    }
                    if ispolicy {
                        let _ = enforcer.add_policy(v.clone()).await;
                    } else {
                        let _ = enforcer.add_grouping_policy(v.clone()).await;
                    }
                }
            }
        }

        Self { enforcer }
    }

    pub async fn addpolice(&mut self, p: Vec<String>) -> Result<bool> {
        self.enforcer.add_policy(p).await
    }
}

pub async fn casbin_enforce(args: Vec<String>) -> Result<bool> {
    GLOBALE_CASBIN_ENFORCER.read().await.enforcer.enforce(args)
}

pub async fn add_policy(p: Vec<String>) -> Result<bool> {
    GLOBALE_CASBIN_ENFORCER.write().await.addpolice(p).await
}

pub async fn remove_policy(p: Vec<String>) -> Result<bool> {
    GLOBALE_CASBIN_ENFORCER
        .write()
        .await
        .enforcer
        .remove_policy(p)
        .await
}

pub async fn add_grouping_policy(gp: Vec<String>) -> Result<bool> {
    GLOBALE_CASBIN_ENFORCER
        .write()
        .await
        .enforcer
        .add_grouping_policy(gp)
        .await
}

pub async fn remove_grouping_policy(gp: Vec<String>) -> Result<bool> {
    GLOBALE_CASBIN_ENFORCER
        .write()
        .await
        .enforcer
        .remove_grouping_policy(gp)
        .await
}

pub async fn get_all_policy() -> Vec<Vec<String>> {
    let vec = GLOBALE_CASBIN_ENFORCER
        .read()
        .await
        .enforcer
        .get_all_policy();

    vec
}
