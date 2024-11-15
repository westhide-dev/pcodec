#![allow(clippy::missing_safety_doc)]

use std::mem::ManuallyDrop;

use libc::{c_uchar, c_uint, c_void, size_t};

use crate::PcoError::PcoInvalidType;
use pco::data_types::{Number, NumberType};
use pco::match_number_enum;

#[repr(C)]
pub enum PcoError {
  PcoSuccess,
  PcoInvalidType,
  // TODO split this into the actual error kinds
  PcoCompressionError,
  PcoDecompressionError,
}

pco::define_number_enum!(
  #[derive()]
  NumVec(Vec)
);

#[repr(C)]
pub struct PcoFfiVec {
  ptr: *const c_void,
  len: size_t,
  cap: size_t,
}

impl PcoFfiVec {
  fn init_from_bytes(&mut self, bytes: Vec<u8>) {
    let m = ManuallyDrop::new(bytes);
    self.ptr = m.as_ptr() as *const c_void;
    self.len = m.len();
    self.cap = m.capacity();
  }

  fn init_from_nums<T: Number>(&mut self, bytes: Vec<T>) {
    let m = ManuallyDrop::new(bytes);
    self.ptr = m.as_ptr() as *const c_void;
    self.len = m.len();
    self.cap = m.capacity();
  }

  fn free<T>(&mut self) {
    unsafe { Vec::<T>::from_raw_parts(self.ptr as *mut T, self.len, self.cap) };
    self.ptr = std::ptr::null();
    self.len = 0;
    self.cap = 0;
  }
}

fn _simpler_compress<T: Number>(
  nums: *const c_void,
  len: size_t,
  level: c_uint,
  ffi_vec_ptr: *mut PcoFfiVec,
) -> PcoError {
  let slice = unsafe { std::slice::from_raw_parts(nums as *const T, len) };
  match pco::standalone::simpler_compress(slice, level as usize) {
    Err(_) => PcoError::PcoCompressionError,
    Ok(v) => {
      unsafe { (*ffi_vec_ptr).init_from_bytes(v) };
      PcoError::PcoSuccess
    }
  }
}

fn _simple_decompress<T: Number>(
  compressed: *const c_void,
  len: size_t,
  ffi_vec_ptr: *mut PcoFfiVec,
) -> PcoError {
  let slice = unsafe { std::slice::from_raw_parts(compressed as *const u8, len) };
  match pco::standalone::simple_decompress::<T>(slice) {
    Err(_) => PcoError::PcoDecompressionError,
    Ok(v) => {
      unsafe { (*ffi_vec_ptr).init_from_nums(v) };
      PcoError::PcoSuccess
    }
  }
}

#[no_mangle]
pub extern "C" fn pco_simple_compress(
  nums: *const c_void,
  len: size_t,
  dtype: c_uchar,
  level: c_uint,
  dst: *mut PcoFfiVec,
) -> PcoError {
  let Some(dtype) = NumberType::from_descriminant(dtype) else {
    return PcoInvalidType;
  };

  match_number_enum!(
    dtype,
    NumberType<T> => {
      _simpler_compress::<T>(nums, len, level, dst)
    }
  )
}

#[no_mangle]
pub extern "C" fn pco_simple_decompress(
  compressed: *const c_void,
  len: size_t,
  dtype: c_uchar,
  dst: *mut PcoFfiVec,
) -> PcoError {
  let Some(dtype) = NumberType::from_descriminant(dtype) else {
    return PcoInvalidType;
  };

  match_number_enum!(
    dtype,
    NumberType<T> => {
      _simple_decompress::<T>(compressed, len, dst)
    }
  )
}

#[no_mangle]
pub unsafe extern "C" fn pco_free_cvec(ffi_vec: *mut PcoFfiVec) -> PcoError {
  unsafe { (*ffi_vec).free::<u8>() };
  PcoError::PcoSuccess
}

#[no_mangle]
pub unsafe extern "C" fn pco_free_dvec(ffi_vec: *mut PcoFfiVec, dtype: c_uchar) -> PcoError {
  let Some(dtype) = NumberType::from_descriminant(dtype) else {
    return PcoInvalidType;
  };

  match_number_enum!(
    dtype,
    NumberType<T> => {
        unsafe { (*ffi_vec).free::<T>() };
    }
  );

  PcoError::PcoSuccess
}
