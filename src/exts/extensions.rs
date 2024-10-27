use deno_runtime::deno_core::{extension, op2};
use ulid::Ulid;

pub const TANXIUM_VERSION: &str = env!("CARGO_PKG_VERSION");

#[op2]
#[string]
pub fn op_tanxium_version() -> String {
    TANXIUM_VERSION.to_string()
}

#[op2]
#[string]
pub fn op_generate_nanoid() -> String {
    nanoid::nanoid!()
}

#[op2]
#[string]
pub fn op_generate_ulid() -> String {
    Ulid::new().to_string()
}

extension!(
    TanxiumExtension,
    ops = [op_generate_nanoid, op_generate_ulid, op_tanxium_version],
    // esm_entry_point = "ext:tanxium.js",
    // esm = [dir "src/exts/js", "tanxium.js"]
);
