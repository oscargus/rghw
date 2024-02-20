use std::ffi::CStr;

use ghw_sys::ghw_type;

pub enum GHWType {
    // Kind(GHDLRTIK),
    Common(GHWTypeCommon),
    /*     Enum(GHWTypeEnum),
       Scalar(GHWTypeScalar),
       Physical(GHWTypePhysical),
       SubtypeScalar(GHWSubtypeScalar),
       Array(GHWTypeArray),
       Record(GHWTypeRecord),
       SubtypeArray(GHWSubtypeArray),
       SubtypeUnboundedArray(GHWSubtypeUnboundedArray),
       SubtypeRecord(GHWSubtypeRecord),
       SubtypeUnboundedRecord(GHWSubtypeUnboundedRecord)
    */
}

pub struct GHWTypeCommon {
    pub handle: *mut ghw_type,
}

impl GHWTypeCommon {
    pub fn name(&self) -> Option<String> {
        unsafe {
            let name = self.handle.as_ref().unwrap().common.name;
            if name.is_null() {
                None
            } else {
                Some(CStr::from_ptr(name).to_str().unwrap().to_string())
            }
        }
    }

    pub fn kind(&self) -> GHDLRTIK {
        unsafe {
            let name = self.handle.as_ref().unwrap().common.kind;
            name.into()
        }
    }
}

pub enum GHWWellKnownType {
  Unknown,
  Boolean,
  Bit,
  StdULogic,
}

impl From<i32> for GHWWellKnownType {
  fn from(value: i32) -> Self {
      match value {
          0 => GHWWellKnownType::Unknown,
          1 => GHWWellKnownType::Boolean,
          2 => GHWWellKnownType::Bit,
          3 => GHWWellKnownType::StdULogic,
          _ => panic!("Cannot convert {} to GHWWellKnownType", value),
      }
  }
}

pub enum GHDLRTIK {
    Top, /* 0  */
    Library,
    Package,
    PackageBody,
    Entity,
    Architecture, /* 5 */
    Process,
    Block,
    IfGenerate,
    ForGenerate,
    Instance,
    Constant,
    Iterator,
    Variable,
    Signal,
    File,
    Port,
    Generic,
    Alias,
    Guard,
    Component,
    Attribute,
    TypeB2, /* 22 */
    TypeE8,
    TypeE32,
    TypeI32, /* 25 */
    TypeI64,
    TypeF64,
    TypeP32,
    TypeP64,
    TypeAccess, /* 30 */
    TypeArray,
    TypeRecord,
    TypeFile,
    SubtypeScalar,
    SubtypeArray,    /* 35 */
    SubtypeArrayPtr, /* Obsolete.  */
    SubtypeUnboundedArray,
    SubtypeRecord,
    SubtypeUnboundedRecord, /* 39 */
    Error,
}

impl From<i32> for GHDLRTIK {
    fn from(value: i32) -> Self {
        match value {
            0 => GHDLRTIK::Top, /* 0  */
            1 => GHDLRTIK::Library,
            2 => GHDLRTIK::Package,
            3 => GHDLRTIK::PackageBody,
            4 => GHDLRTIK::Entity,
            5 => GHDLRTIK::Architecture, /* 5 */
            6 => GHDLRTIK::Process,
            7 => GHDLRTIK::Block,
            8 => GHDLRTIK::IfGenerate,
            9 => GHDLRTIK::ForGenerate,
            10 => GHDLRTIK::Instance,
            11 => GHDLRTIK::Constant,
            12 => GHDLRTIK::Iterator,
            13 => GHDLRTIK::Variable,
            14 => GHDLRTIK::Signal,
            15 => GHDLRTIK::File,
            16 => GHDLRTIK::Port,
            17 => GHDLRTIK::Generic,
            18 => GHDLRTIK::Alias,
            19 => GHDLRTIK::Guard,
            20 => GHDLRTIK::Component,
            21 => GHDLRTIK::Attribute,
            22 => GHDLRTIK::TypeB2, /* 22 */
            23 => GHDLRTIK::TypeE8,
            24 => GHDLRTIK::TypeE32,
            25 => GHDLRTIK::TypeI32, /* 25 */
            26 => GHDLRTIK::TypeI64,
            27 => GHDLRTIK::TypeF64,
            28 => GHDLRTIK::TypeP32,
            29 => GHDLRTIK::TypeP64,
            30 => GHDLRTIK::TypeAccess, /* 30 */
            31 => GHDLRTIK::TypeArray,
            32 => GHDLRTIK::TypeRecord,
            33 => GHDLRTIK::TypeFile,
            34 => GHDLRTIK::SubtypeScalar,
            35 => GHDLRTIK::SubtypeArray,    /* 35 */
            36 => GHDLRTIK::SubtypeArrayPtr, /* Obsolete.  */
            37 => GHDLRTIK::SubtypeUnboundedArray,
            38 => GHDLRTIK::SubtypeRecord,
            39 => GHDLRTIK::SubtypeUnboundedRecord, /* 39 */
            40 => GHDLRTIK::Error,
            _ => panic!("Cannot convert {} to GHDLRTIK", value),
        }
    }
}

/* struct ghw_type_common
{
  enum ghdl_rtik kind;
  const char *name;
};

struct ghw_type_enum
{
  enum ghdl_rtik kind;
  const char *name;

  enum ghw_wkt_type wkt;
  uint32_t nbr;
  const char **lits;
};

struct ghw_type_scalar
{
  enum ghdl_rtik kind;
  const char *name;
};

struct ghw_unit
{
  const char *name;
  int64_t val;
};

struct ghw_type_physical
{
  enum ghdl_rtik kind;
  const char *name;
  uint32_t nbr_units;
  struct ghw_unit *units;
};

struct ghw_type_array
{
  enum ghdl_rtik kind;
  const char *name;

  unsigned int nbr_dim;
  union ghw_type *el;
  union ghw_type **dims;
};

struct ghw_subtype_unbounded_array
{
  enum ghdl_rtik kind;
  const char *name;

  union ghw_type *base;
};

struct ghw_subtype_array
{
  enum ghdl_rtik kind;
  const char *name;

  union ghw_type *base;
  int nbr_scalars;
  union ghw_range **rngs;
  union ghw_type *el;
};

struct ghw_subtype_scalar
{
  enum ghdl_rtik kind;
  const char *name;

  union ghw_type *base;
  union ghw_range *rng;
};

struct ghw_record_element
{
  const char *name;
  union ghw_type *type;
};

struct ghw_type_record
{
  enum ghdl_rtik kind;
  const char *name;

  unsigned int nbr_fields;
  int nbr_scalars;		/* Number of scalar elements (ie nbr of signals).  */
  struct ghw_record_element *els;
};

struct ghw_subtype_record
{
  enum ghdl_rtik kind;
  const char *name;

  struct ghw_type_record *base;
  int nbr_scalars;		/* Number of scalar elements (ie nbr of signals).  */
  struct ghw_record_element *els;
};

struct ghw_subtype_unbounded_record
{
  enum ghdl_rtik kind;
  const char *name;

  struct ghw_type_record *base;
};
 */
