use std::ptr::null_mut;

use llvm_sys::{
    core::{
        LLVMAppendBasicBlockInContext, LLVMArrayType2, LLVMBFloatTypeInContext, LLVMContextCreate,
        LLVMContextDispose, LLVMCreateBuilderInContext, LLVMDoubleTypeInContext,
        LLVMFP128TypeInContext, LLVMFloatTypeInContext, LLVMFunctionType, LLVMHalfTypeInContext,
        LLVMInt128TypeInContext, LLVMInt16TypeInContext, LLVMInt1TypeInContext,
        LLVMInt32TypeInContext, LLVMInt64TypeInContext, LLVMInt8TypeInContext,
        LLVMIntTypeInContext, LLVMModuleCreateWithNameInContext, LLVMPPCFP128TypeInContext,
        LLVMPointerTypeInContext, LLVMStructCreateNamed, LLVMStructTypeInContext,
        LLVMVoidTypeInContext, LLVMX86FP80TypeInContext,
    },
    prelude::{LLVMContextRef, LLVMTypeRef},
};

use crate::{
    basic_block::BasicBlock,
    builder::Builder,
    module::Module,
    ty::{AddressSpace, Type},
    util::string_to_cstring,
    value::Value,
};

#[derive(Debug, Clone)]
pub struct Context(LLVMContextRef);

impl Context {
    /// Get inner [`LLVMContextRef`].
    pub(crate) fn get(&self) -> LLVMContextRef {
        self.0
    }

    /// Create a new [`Context`].
    pub fn create() -> Self {
        unsafe {
            let pointer = LLVMContextCreate();
            assert_ne!(pointer, null_mut(), "context pointer is null");
            Self(pointer)
        }
    }

    /// Create a new [`Module`] in the current [`Context`].
    pub fn create_module<S: ToString>(&self, name: S) -> Module {
        let name = string_to_cstring(name.to_string());

        unsafe { Module::new(LLVMModuleCreateWithNameInContext(name.as_ptr(), self.get())) }
    }

    /// Create a new [`Builder`] in the current [`Context`].
    pub fn create_builder(&self) -> Builder {
        unsafe { Builder::new(LLVMCreateBuilderInContext(self.get())) }
    }

    /// Create a new Void [`Type`].
    pub fn create_void_type(&self) -> Type {
        unsafe { Type::new(LLVMVoidTypeInContext(self.get())) }
    }

    /// Create a new 1-bit Integer [`Type`].
    pub fn create_i1_type(&self) -> Type {
        unsafe { Type::new(LLVMInt1TypeInContext(self.get())) }
    }

    /// Create a new 8-bit Integer [`Type`].
    pub fn create_i8_type(&self) -> Type {
        unsafe { Type::new(LLVMInt8TypeInContext(self.get())) }
    }

    /// Create a new 16-bit Integer [`Type`].
    pub fn create_i16_type(&self) -> Type {
        unsafe { Type::new(LLVMInt16TypeInContext(self.get())) }
    }

    /// Create a new 32-bit Integer [`Type`].
    pub fn create_i32_type(&self) -> Type {
        unsafe { Type::new(LLVMInt32TypeInContext(self.get())) }
    }

    /// Create a new 64-bit Integer [`Type`].
    pub fn create_i64_type(&self) -> Type {
        unsafe { Type::new(LLVMInt64TypeInContext(self.get())) }
    }

    /// Create a new 128-bit Integer [`Type`].
    pub fn create_i128_type(&self) -> Type {
        unsafe { Type::new(LLVMInt128TypeInContext(self.get())) }
    }

    /// Create a new n-bit Integer [`Type`].
    pub fn create_int_type(&self, n: u32) -> Type {
        unsafe { Type::new(LLVMIntTypeInContext(self.get(), n)) }
    }

    /// Create a new 16-bit brain Float [`Type`].
    pub fn create_bf16_type(&self) -> Type {
        unsafe { Type::new(LLVMBFloatTypeInContext(self.get())) }
    }

    /// Create a new 16-bit Float [`Type`].
    pub fn create_f16_type(&self) -> Type {
        unsafe { Type::new(LLVMHalfTypeInContext(self.get())) }
    }

    /// Create a new 32-bit Float [`Type`].
    pub fn create_f32_type(&self) -> Type {
        unsafe { Type::new(LLVMFloatTypeInContext(self.get())) }
    }

    /// Create a new 64-bit Float [`Type`].
    pub fn create_f64_type(&self) -> Type {
        unsafe { Type::new(LLVMDoubleTypeInContext(self.get())) }
    }

    /// Create a new 80-bit Float [`Type`] on x86 only.
    pub fn create_x86_f80_type(&self) -> Type {
        unsafe { Type::new(LLVMX86FP80TypeInContext(self.get())) }
    }

    /// Create a new 128-bit Float [`Type`].
    pub fn create_f128_type(&self) -> Type {
        unsafe { Type::new(LLVMFP128TypeInContext(self.get())) }
    }

    /// Create a new 128-bit Float [`Type`], split into 2x 64-bit on PowerPC only.
    pub fn create_ppc_f128_type(&self) -> Type {
        unsafe { Type::new(LLVMPPCFP128TypeInContext(self.get())) }
    }

    /// Create a new Pointer [`Type`].
    pub fn create_ptr_type(&self, address_space: AddressSpace) -> Type {
        unsafe { Type::new(LLVMPointerTypeInContext(self.get(), address_space as u32)) }
    }

    /// Create a new Function [`Type`].
    pub fn create_func_type(&self, return_ty: &Type, param_tys: &[Type], is_var_arg: bool) -> Type {
        let mut param_tys = param_tys
            .iter()
            .map(|ty| ty.get())
            .collect::<Vec<LLVMTypeRef>>();

        unsafe {
            Type::new(LLVMFunctionType(
                return_ty.get(),
                param_tys.as_mut_ptr(),
                param_tys.len() as u32,
                is_var_arg as i32,
            ))
        }
    }

    /// Create a new Array [`Type`].
    pub fn create_array_type(&self, element_ty: &Type, size: u64) -> Type {
        unsafe { Type::new(LLVMArrayType2(element_ty.get(), size)) }
    }

    /// Create a new Struct [`Type`].
    pub fn create_struct_type(&self, element_tys: &[Type], is_packed: bool) -> Type {
        let mut element_tys = element_tys
            .iter()
            .map(|elem| elem.get())
            .collect::<Vec<LLVMTypeRef>>();

        unsafe {
            Type::new(LLVMStructTypeInContext(
                self.get(),
                element_tys.as_mut_ptr(),
                element_tys.len() as u32,
                is_packed as i32,
            ))
        }
    }

    /// Create a named Struct [`Type`].
    pub fn create_named_struct_type<S: ToString>(&self, name: S) -> Type {
        let name = string_to_cstring(name.to_string());

        unsafe { Type::new(LLVMStructCreateNamed(self.get(), name.as_ptr())) }
    }

    /// Appends a [`BasicBlock`] to a function.
    pub fn append_basic_block<S: ToString>(&self, func: &Value, name: S) -> BasicBlock {
        let name = string_to_cstring(name.to_string());

        unsafe {
            BasicBlock::new(LLVMAppendBasicBlockInContext(
                self.get(),
                func.get(),
                name.as_ptr(),
            ))
        }
    }

    /// Dispose this [`Context`] and free its memory.
    pub fn dispose(self) {
        unsafe { LLVMContextDispose(self.get()) }
    }
}
