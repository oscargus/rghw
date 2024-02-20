use core::fmt;
use std::ffi::CStr;

use ghw_sys::ghw_hie;

use crate::types::GHDLRTIK;

#[derive(Debug, Clone)]
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

    pub fn name(&self) -> Option<String> {
        unsafe {
            let name = self.handle.as_ref().unwrap().name;
            if name.is_null() {
                None
            } else {
                Some(CStr::from_ptr(name).to_str().unwrap().to_string())
            }
        }
    }

    pub fn kind(&self) -> GHWHierarchyKind {
        let kind = unsafe { self.handle.as_ref().unwrap().kind };
        kind.into()
    }

    pub fn children(&self) -> Vec<GHWHierarchy> {
        let mut ret: Vec<GHWHierarchy> = Vec::new();
        match self.kind() {
            GHWHierarchyKind::Design
            | GHWHierarchyKind::Block
            | GHWHierarchyKind::GenerateFor
            | GHWHierarchyKind::GenerateIf
            | GHWHierarchyKind::Instance
            | GHWHierarchyKind::Package
            | GHWHierarchyKind::Process => unsafe {
                let handle = self.handle.as_ref().unwrap().u.blk.child;
                if !handle.is_null() {
                    let mut hier = GHWHierarchy { handle };
                    ret.push(hier.clone());
                    loop {
                        let brother = hier.brother();
                        if let Some(brother) = brother {
                            ret.push(brother.clone());
                            hier = brother;
                        } else {
                            break;
                        }
                    }
                }
            },
            GHWHierarchyKind::Signal
            | GHWHierarchyKind::PortBuffer
            | GHWHierarchyKind::PortIn
            | GHWHierarchyKind::PortInOut
            | GHWHierarchyKind::PortLinkage
            | GHWHierarchyKind::PortOut => unsafe {
                let sigs = self.handle.as_ref().unwrap().u.sig.sigs;
                let subtype = self.handle.as_ref().unwrap().u.sig.type_;
                let kind: GHDLRTIK = (*subtype).kind.into();
            },
            _ => panic!("Unhandled hierarchy kind: {}", self.kind()),
        }
        ret
    }

    pub fn child_scopes(&self) -> Vec<GHWHierarchy> {
        self.children()
            .iter()
            .filter(|x| match x.kind() {
                GHWHierarchyKind::Design
                | GHWHierarchyKind::Block
                | GHWHierarchyKind::GenerateFor
                | GHWHierarchyKind::GenerateIf
                | GHWHierarchyKind::Instance
                | GHWHierarchyKind::Package
                | GHWHierarchyKind::Process => true,
                _ => false,
            })
            .cloned()
            .collect()
    }
}

#[derive(Debug, PartialEq)]
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
    EndOfSection,
    Signal,
    PortIn,
    PortOut,
    PortInOut,
    PortBuffer,
    PortLinkage,
}

impl fmt::Display for GHWHierarchyKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GHWHierarchyKind::EOH => write!(f, "eoh"),
            GHWHierarchyKind::Design => write!(f, "design"),
            GHWHierarchyKind::Block => write!(f, "block"),
            GHWHierarchyKind::GenerateIf => write!(f, "generate-if"),
            GHWHierarchyKind::GenerateFor => write!(f, "generate-for"),
            GHWHierarchyKind::Instance => write!(f, "instance"),
            GHWHierarchyKind::Package => write!(f, "package"),
            GHWHierarchyKind::Process => write!(f, "process"),
            GHWHierarchyKind::Generic => write!(f, "generic"),
            GHWHierarchyKind::EndOfSection => write!(f, "eos"),
            GHWHierarchyKind::Signal => write!(f, "signal"),
            GHWHierarchyKind::PortIn => write!(f, "port-in"),
            GHWHierarchyKind::PortOut => write!(f, "port-out"),
            GHWHierarchyKind::PortInOut => write!(f, "port-inout"),
            GHWHierarchyKind::PortBuffer => write!(f, "port-buffer"),
            GHWHierarchyKind::PortLinkage => write!(f, "port-linkage"),
        }
    }
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
            15 => GHWHierarchyKind::EndOfSection,
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





