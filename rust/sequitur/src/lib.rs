#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{c_void, CStr, CString};
use std::os::raw::{c_char, c_int};
use std::slice;

pub mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub struct MultigramInventory {
    ptr: *mut ffi::MultigramInventory_t,
}

impl MultigramInventory {
    pub fn new() -> Self {
        unsafe {
            let ptr = ffi::MultigramInventory_new();
            MultigramInventory { ptr }
        }
    }

    pub fn size(&self) -> i32 {
        unsafe { ffi::MultigramInventory_size(self.ptr) }
    }
}

impl Drop for MultigramInventory {
    fn drop(&mut self) {
        unsafe {
            ffi::MultigramInventory_delete(self.ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// This is a simple integration test that checks if the Rust wrappers can
    /// create and interact with the C++ objects.
    ///
    /// To run this test, you will need:
    /// - A C++ compiler
    /// - Rust and Cargo
    /// - The dependencies of the sequitur-g2p project, including numpy.
    ///
    /// You can run the test with the command `cargo test`.
    ///
    /// Note: This test will not run in the current environment because of issues
    /// with the `run_in_bash_session` tool. The user will have to run it in their
    /// own environment.
    #[test]
    fn test_translation() {
        let inventory = MultigramInventory::new();
        let model = SequenceModel::new();
        let mut translator = Translator::new();
        translator.set_multigram_inventory(&inventory);
        translator.set_sequence_model(&model);

        let input = vec![];
        let output = translator.translate(&input);
        assert_eq!(output, vec![]);
    }
}

pub struct SequenceModel {
    ptr: *mut ffi::SequenceModel_t,
}

impl SequenceModel {
    pub fn new() -> Self {
        unsafe {
            let ptr = ffi::SequenceModel_new();
            SequenceModel { ptr }
        }
    }
}

impl Drop for SequenceModel {
    fn drop(&mut self) {
        unsafe {
            ffi::SequenceModel_delete(self.ptr);
        }
    }
}

pub struct EstimationGraphBuilder {
    ptr: *mut ffi::EstimationGraphBuilder_t,
}

impl EstimationGraphBuilder {
    pub fn new() -> Self {
        unsafe {
            let ptr = ffi::EstimationGraphBuilder_new();
            EstimationGraphBuilder { ptr }
        }
    }

    pub fn set_sequence_model(&mut self, inventory: &MultigramInventory, model: &SequenceModel) {
        unsafe {
            ffi::EstimationGraphBuilder_setSequenceModel(self.ptr, inventory.ptr, model.ptr);
        }
    }
}

impl Drop for EstimationGraphBuilder {
    fn drop(&mut self) {
        unsafe {
            ffi::EstimationGraphBuilder_delete(self.ptr);
        }
    }
}

pub struct Translator {
    ptr: *mut ffi::Translator_t,
}

impl Translator {
    pub fn new() -> Self {
        unsafe {
            let ptr = ffi::Translator_new();
            Translator { ptr }
        }
    }

    pub fn set_multigram_inventory(&mut self, inventory: &MultigramInventory) {
        unsafe {
            ffi::Translator_setMultigramInventory(self.ptr, inventory.ptr);
        }
    }

    pub fn set_sequence_model(&mut self, model: &SequenceModel) {
        unsafe {
            ffi::Translator_setSequenceModel(self.ptr, model.ptr);
        }
    }

    pub fn translate(&self, seq: &[i32]) -> Vec<i32> {
        let mut out_len: c_int = 0;
        unsafe {
            let out_ptr = ffi::Translator_translate(
                self.ptr,
                seq.as_ptr(),
                seq.len() as c_int,
                &mut out_len,
            );
            let result = slice::from_raw_parts(out_ptr, out_len as usize).to_vec();
            ffi::free_sequence(out_ptr);
            result
        }
    }
}

impl Drop for Translator {
    fn drop(&mut self) {
        unsafe {
            ffi::Translator_delete(self.ptr);
        }
    }
}
