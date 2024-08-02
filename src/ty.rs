use std::{ffi::CString, ptr::null_mut};

use llvm_sys::{
    core::{
        LLVMCountStructElementTypes, LLVMGetArrayLength2, LLVMGetElementType, LLVMGetIntTypeWidth,
        LLVMGetStructElementTypes, LLVMGetStructName, LLVMGetTypeKind, LLVMIsLiteralStruct,
        LLVMIsOpaqueStruct, LLVMIsPackedStruct, LLVMPointerTypeIsOpaque, LLVMStructGetTypeAtIndex,
    },
    prelude::LLVMTypeRef,
    LLVMTypeKind,
};

use crate::util::cstring_to_string;

#[derive(Debug, Clone, Copy)]
pub struct Type(LLVMTypeRef);

impl Type {
    /// Creates a [`Type`] from a [`LLVMTypeRef`].
    pub(crate) fn new(pointer: LLVMTypeRef) -> Self {
        assert_ne!(pointer, null_mut(), "type pointer is null");
        Self(pointer)
    }

    #[inline]
    /// Get inner [`LLVMTypeRef`].
    pub(crate) fn get(&self) -> LLVMTypeRef {
        self.0
    }

    /// Get [`TypeKind`] of this [`Type`].
    pub fn get_type_kind(&self) -> LLVMTypeKind {
        unsafe { LLVMGetTypeKind(self.get()).into() }
    }

    /// Get name of Struct [`Type`].
    pub fn get_struct_name(&self) -> String {
        let name = unsafe { CString::from_raw(LLVMGetStructName(self.get()) as *mut _) };
        cstring_to_string(name)
    }

    /// Get struct element count.
    pub fn get_struct_elemen_count(&self) -> usize {
        unsafe { LLVMCountStructElementTypes(self.get()) as usize }
    }

    /// Get struct element [`Type`]s.
    pub fn get_struct_element_tys(&self) -> Vec<Type> {
        let element_count = self.get_struct_elemen_count();
        let mut element_tys = vec![null_mut(); element_count];

        unsafe { LLVMGetStructElementTypes(self.get(), element_tys.as_mut_ptr()) };

        element_tys.into_iter().map(|ty| Type::new(ty)).collect()
    }

    /// Get struct element [`Type`] at index.
    pub fn get_struct_element_ty(&self, index: usize) -> Option<Type> {
        let ty = unsafe { LLVMStructGetTypeAtIndex(self.get(), index as u32) };

        if ty == null_mut() {
            return None;
        }

        Some(Type::new(ty))
    }

    /// Is struct packed.
    pub fn is_struct_packed(&self) -> bool {
        unsafe { LLVMIsPackedStruct(self.get()) == 1 }
    }

    /// Is struct opaque.
    pub fn is_struct_opaque(&self) -> bool {
        unsafe { LLVMIsOpaqueStruct(self.get()) == 1 }
    }

    /// Is struct literal.
    pub fn is_struct_literal(&self) -> bool {
        unsafe { LLVMIsLiteralStruct(self.get()) == 1 }
    }

    /// Get Integer [`Type`] bit width.
    pub fn get_int_type_width(&self) -> u32 {
        unsafe { LLVMGetIntTypeWidth(self.get()) }
    }

    /// Get element [`Type`].
    pub fn get_element_type(&self) -> Type {
        unsafe { Type::new(LLVMGetElementType(self.get())) }
    }

    /// Get array length [`Type`].
    pub fn get_array_length(&self) -> usize {
        unsafe { LLVMGetArrayLength2(self.get()) as usize }
    }

    /// Is pointer opaque.
    pub fn is_pointer_opaque(&self) -> bool {
        unsafe { LLVMPointerTypeIsOpaque(self.get()) == 1 }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
/// See https://llvm.org/doxygen/NVPTXBaseInfo_8h_source.html
pub enum AddressSpace {
    Generic = 0,
    Global = 1,
    Shared = 3,
    Const = 4,
    Local = 5,
    Param = 101,
}
