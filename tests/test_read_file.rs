use std::path::PathBuf;

use rs_ghw::{handle::GHWHandle, hierarchy::GHWHierarchyKind};

fn ghw_asset(rel_path: &str) -> PathBuf {
    let mut path = PathBuf::from(file!());
    path.pop();
    path.pop();
    path.push("assets");
    path.push(rel_path);
    path
}

#[test]
pub fn test_read_file_lib() {
    let ghw_path = ghw_asset("adder_tb.ghw");

    let mut ghw_h = GHWHandle::from_file(ghw_path.to_str().unwrap());
    ghw_h.set_full_names(true);
    ghw_h.read_base();
    ghw_h.disp_values();
    ghw_h.disp_types();
    let hierarchy = ghw_h.hierarchy();
    ghw_h.disp_hierarchy(hierarchy.clone());
    assert_eq!(ghw_h.number_of_signals(), 6);
    assert_eq!(ghw_h.number_of_strings(), 14);
    assert_eq!(ghw_h.number_of_types(), 1);
    assert_eq!(hierarchy.name(), None);
    assert_eq!(hierarchy.kind(), GHWHierarchyKind::Design);
    let children = hierarchy.children();
    assert_eq!(children.len(), 2);
    let pack = &children[0];
    assert_eq!(pack.kind(), GHWHierarchyKind::Package);
    assert_eq!(pack.name(), Some("standard".to_string()));
    assert_eq!(pack.children().len(), 0);
    let inst = &children[1];
    assert_eq!(inst.kind(), GHWHierarchyKind::Instance);
    assert_eq!(inst.children().len(), 7);
    assert_eq!(inst.name(), Some("adder_tb".to_string()));
    assert_eq!(inst.child_scopes().len(), 2);
    assert_eq!(inst.child_variables().len(), 5);
    // assert_eq!(ghw_h.next_time(), 0);
    ghw_h.close();
}

#[test]
pub fn test_read_file_vhdl_features() {
    let ghw_path = ghw_asset("vhdl.ghw");

    let mut ghw_h = GHWHandle::from_file(ghw_path.to_str().unwrap());
    ghw_h.set_full_names(true);
    ghw_h.read_base();
    ghw_h.disp_values();
    ghw_h.disp_types();
    let hierarchy = ghw_h.hierarchy();
    ghw_h.disp_hierarchy(hierarchy.clone());
    assert_eq!(ghw_h.number_of_signals(), 12);
    assert_eq!(ghw_h.number_of_strings(), 29);
    assert_eq!(ghw_h.number_of_types(), 9);
    assert_eq!(hierarchy.name(), None);
    assert_eq!(hierarchy.kind(), GHWHierarchyKind::Design);
    let children = hierarchy.children();
    assert_eq!(children.len(), 3);
    let pack = &children[0];
    assert_eq!(pack.kind(), GHWHierarchyKind::Package);
    assert_eq!(pack.name(), Some("standard".to_string()));
    assert_eq!(pack.children().len(), 0);
    let inst = &children[1];
    assert_eq!(inst.kind(), GHWHierarchyKind::Package);
    assert_eq!(inst.children().len(), 0);
    assert_eq!(inst.name(), Some("std_logic_1164".to_string()));
    assert_eq!(inst.child_scopes().len(), 0);
    assert_eq!(inst.child_variables().len(), 0);
    let inst = &children[2];
    assert_eq!(inst.kind(), GHWHierarchyKind::Instance);
    assert_eq!(inst.children().len(), 3);
    assert_eq!(inst.name(), Some("test".to_string()));
    assert_eq!(inst.child_scopes().len(), 1);
    assert_eq!(inst.child_variables().len(), 2);
    // assert_eq!(ghw_h.next_time(), 0);
    ghw_h.close();
}
