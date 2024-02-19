use std::path::PathBuf;

use rs_ghw::ghwhandle::{GHWHandle, GHWHierarchyKind};

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
    //ghw_h.disp_values();
    //ghw_h.disp_types();
    let hierarchy = ghw_h.hierarchy();
    ghw_h.disp_hierarchy(hierarchy.clone());
    assert_eq!(ghw_h.number_of_signals(), 6);
    assert_eq!(ghw_h.number_of_strings(), 14);
    assert_eq!(ghw_h.number_of_types(), 1);
    assert_eq!(hierarchy.name(), None);
    assert_eq!(hierarchy.kind(), GHWHierarchyKind::Design);
    let children = hierarchy.children();
    assert_eq!(children.len(), 2);
    if children[0].kind() == GHWHierarchyKind::Package {
        assert_eq!(children[0].name(), Some("standard".to_string()));
        assert_eq!(children[0].children().len(), 0);
    } else {
        assert_eq!(children[0].children().len(), 7);
        assert_eq!(children[0].name(), Some("adder_tb".to_string()));
    }
    if children[1].kind() == GHWHierarchyKind::Package {
        assert_eq!(children[1].name(), Some("standard".to_string()));
        assert_eq!(children[1].children().len(), 0);
    } else {
        assert_eq!(children[1].name(), Some("adder_tb".to_string()));
        assert_eq!(children[1].children().len(), 7);
    }
    ghw_h.close();
}
