use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/embedfiles/"]
struct Asset;

pub fn get_rbac_model() -> Option<rust_embed::EmbeddedFile> {
    Asset::get("rbac_with_domains_model.conf")
}

pub fn get_rbac_policy() -> Option<rust_embed::EmbeddedFile> {
    Asset::get("rbac_with_domains_policy.csv")
}
