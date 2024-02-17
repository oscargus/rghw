use std::ffi::{CStr, CString};

use crate::{
    ghw_close, ghw_disp_hie, ghw_disp_types, ghw_disp_values, ghw_handler, ghw_hie, ghw_open,
    ghw_read_base,
};

pub struct GHWHandle {
    handle: std::mem::MaybeUninit<ghw_handler>,
    filename: Option<String>,
}

impl GHWHandle {
    pub fn new() -> Self {
        GHWHandle {
            handle: std::mem::MaybeUninit::uninit(),
            filename: None,
        }
    }

    pub fn from_file(filename: &str) -> Self {
        let mut h = GHWHandle::new();
        h.filename = Some(filename.to_string());
        let path = CString::new(filename).unwrap();
        let ptr = path.as_ptr();
        unsafe {
            ghw_open(h.handle.as_mut_ptr(), ptr);
            h
        }
    }

    pub fn from_file_verbose(filename: &str) -> Self {
        let mut h = GHWHandle::new();
        h.set_verbose(true);
        h.filename = Some(filename.to_string());
        let path = CString::new(filename).unwrap();
        let ptr = path.as_ptr();
        unsafe {
            ghw_open(h.handle.as_mut_ptr(), ptr);
            h
        }
    }

    pub fn read_base(&mut self) -> i32 {
        unsafe { ghw_read_base(self.handle.as_mut_ptr()) }
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        unsafe {
            (*self.handle.as_mut_ptr()).flag_verbose = verbose as i32;
        }
    }

    pub fn disp_values(&mut self) {
        unsafe {
            ghw_disp_values(self.handle.as_mut_ptr());
        }
    }

    pub fn disp_types(&mut self) {
        unsafe {
            ghw_disp_types(self.handle.as_mut_ptr());
        }
    }

    pub fn close(&mut self) {
        unsafe {
            ghw_close(self.handle.as_mut_ptr());
        }
    }

    pub fn hierarchy(&mut self) -> GHWHierarchy {
        unsafe {
            GHWHierarchy {
                handle: (*self.handle.as_mut_ptr()).hie,
            }
        }
    }

    pub fn disp_hierarchy(&mut self, hierarchy: GHWHierarchy) {
        unsafe {
            ghw_disp_hie(self.handle.as_mut_ptr(), hierarchy.handle);
        }
    }

    pub fn set_full_names(&mut self, full_names: bool) {
        unsafe {
            (*self.handle.as_mut_ptr()).flag_full_names = full_names as i32;
        }
    }
}

#[derive(Debug)]
pub struct GHWHierarchy {
    pub handle: *mut ghw_hie,
}

impl GHWHierarchy {
    pub fn parent(&self) -> Option<GHWHierarchy> {
        unsafe {
            let parent = self.handle.as_ref().unwrap().parent;
            if parent.is_null() {
                None
            } else {
                Some(GHWHierarchy { handle: parent })
            }
        }
    }

    pub fn brother(&self) -> Option<GHWHierarchy> {
        unsafe {
            let brother = self.handle.as_ref().unwrap().brother;
            if brother.is_null() {
                None
            } else {
                Some(GHWHierarchy { handle: brother })
            }
        }
    }

    pub fn name(&self) -> String {
        unsafe {
            let name = self.handle.as_ref().unwrap().name;
            CStr::from_ptr(name).to_str().unwrap().to_string()
        }
    }

    pub fn kind(&self) -> GHWHierarchyKind {
        let kind = unsafe { self.handle.as_ref().unwrap().kind };
        kind.into()
    }
}

pub enum GHWHierarchyKind {
    EOH,
    Design,
    Block,
    GenerateIf,
    GenerateFor,
    Instance,
    Package,
    Process,
    Generic,
    EOS,
    Signal,
    PortIn,
    PortOut,
    PortInOut,
    PortBuffer,
    PortLinkage,
}

impl From<i32> for GHWHierarchyKind {
    fn from(num: i32) -> GHWHierarchyKind {
        match num {
            0 => GHWHierarchyKind::EOH,
            1 => GHWHierarchyKind::Design,
            3 => GHWHierarchyKind::Block,
            4 => GHWHierarchyKind::GenerateIf,
            5 => GHWHierarchyKind::GenerateFor,
            6 => GHWHierarchyKind::Instance,
            7 => GHWHierarchyKind::Package,
            13 => GHWHierarchyKind::Process,
            14 => GHWHierarchyKind::Generic,
            15 => GHWHierarchyKind::EOS,
            16 => GHWHierarchyKind::Signal,
            17 => GHWHierarchyKind::PortIn,
            18 => GHWHierarchyKind::PortOut,
            19 => GHWHierarchyKind::PortInOut,
            20 => GHWHierarchyKind::PortBuffer,
            21 => GHWHierarchyKind::PortLinkage,
            _ => panic!("Cannot convert {} to GHWHierarchyKind", num),
        }
    }
}

pub enum GHWWellKnownType {
    Unknown,
    Boolean,
    Bit,
    StdULogic
}

impl From<i32> for GHWWellKnownType {
    fn from(value: i32) -> Self {
        match value {
            0 => GHWWellKnownType::Unknown,
            1 => GHWWellKnownType::Boolean,
            2 => GHWWellKnownType::Bit,
            3 => GHWWellKnownType::StdULogic,
            _ => panic!("Cannot convert {} to GHWWellKnownType", value)
        }
    }
}