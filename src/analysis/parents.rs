use std::collections::HashSet;
use std::vec::Vec;

use analysis::rust_type::used_rust_type;
use env::Env;
use super::general::StatusedTypeId;
use gobjects::*;
use library::Class;
use traits::*;

pub fn analyze(env: &Env, type_: &Class, used_types: &mut HashSet<String>)
    -> (Vec<StatusedTypeId>, bool) {
    let mut parents = Vec::new();
    let mut has_ignored_parents = false;

    for &parent_tid in &type_.parents {
        let parent_type = env.type_(parent_tid).to_ref_as::<Class>();

        let status = env.type_status(&parent_tid.full_name(&env.library));

        parents.push(StatusedTypeId{
            type_id: parent_tid,
            name: parent_type.name.clone(),
            status: status,
        });
        used_rust_type(env, parent_tid).ok().map(|s| used_types.insert(s));

        if status == GStatus::Ignore { has_ignored_parents = true; }

        if parent_type.c_type == "GtkWidget" { break }
    }
    parents.reverse();

    (parents, has_ignored_parents)
}