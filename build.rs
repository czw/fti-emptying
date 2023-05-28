// SPDX-License-Identifier: MIT

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=i18n");
}
