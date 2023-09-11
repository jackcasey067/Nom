
use crate::instructions::IntSize;


#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Type {
    BuiltIn (BuiltIn),
    
    #[allow(dead_code)]
    NotYetImplemented,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum BuiltIn {
    U8,
    U16,
    U32,
    U64, 
    I8,
    I16,
    I32, 
    I64,
    Unit,
}

impl BuiltIn {
    pub fn is_signed(&self) -> bool {
        use BuiltIn as B;
       
        matches!(self, B::I8 | B::I16 | B::I32 | B::I64)
    }

    pub fn is_unsigned(&self) -> bool {
        use BuiltIn as B;
       
        matches!(self, B::U8 | B::U16 | B::U32 | B::U64)
    }

    pub fn get_int_size(&self) -> Option<IntSize> {
        use BuiltIn as B;
        use IntSize as IS;

        match self {
            B::U8 | B::I8 => Some(IS::OneByte),
            B::U16 | B::I16 => Some(IS::TwoByte),
            B::U32 | B::I32 => Some(IS::FourByte),
            B::U64 | B::I64 => Some(IS::EightByte),
            _ => None
        }
    }
}

impl From<String> for Type {
    fn from(value: String) -> Self {
        match &value[..] {
            "i8" => Type::BuiltIn(BuiltIn::I8),
            "i16" => Type::BuiltIn(BuiltIn::I16),
            "i32" => Type::BuiltIn(BuiltIn::I32),
            "i64" => Type::BuiltIn(BuiltIn::I64),
            "u8" => Type::BuiltIn(BuiltIn::U8),
            "u16" => Type::BuiltIn(BuiltIn::U16),
            "u32" => Type::BuiltIn(BuiltIn::U32),
            "u64" => Type::BuiltIn(BuiltIn::U64),
            "unit" => Type::BuiltIn(BuiltIn::Unit),
            _ => panic!("User defined type not yet implemented"),
        }
    }
}

#[derive(Debug)]
pub struct TypeInfo {
    pub size: usize,  // Number of bytes the types takes on the stack.
    pub alignment: usize,  // In bytes
}
