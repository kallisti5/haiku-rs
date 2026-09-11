//
// Copyright 2018, 2024, Niels Sascha Reedijk <niels.reedijk@gmail.com>
// All rights reserved. Distributed under the terms of the MIT License.
//

//! Module for flattening and unflattening data
//!
//! Flattening is a Haiku concept where all types of data can be stored as and
//! read from a byte stream. It is used in several areas, such as Messages and
//! file attributes. This module implements the concept for Rust, which makes
//! it possible to work with flattened data in Rust. If you want to use the
//! flattening API for your own data, you should implement the Flattenable
//! trait.

use std::ffi::{CStr, CString};

use libc::{
	B_BOOL_TYPE, B_DOUBLE_TYPE, B_FLOAT_TYPE, B_INT16_TYPE, B_INT32_TYPE, B_INT64_TYPE,
	B_INT8_TYPE, B_STRING_TYPE, B_UINT16_TYPE, B_UINT32_TYPE, B_UINT64_TYPE, B_UINT8_TYPE,
};

use crate::support::{ErrorKind, HaikuError, Result};

/// An interface for types that are flattenable
pub trait Flattenable<T> {
	/// The type code is a unique identifier that identifies the flattened data
	fn type_code() -> u32;
	/// Check if flattened objects of this type are always a fixed size
	fn is_fixed_size() -> bool;
	/// Return the size of the flattened type
	fn flattened_size(&self) -> usize;
	/// Return a flattened version of this object
	fn flatten(&self) -> Vec<u8>;
	/// Unflatten an object from a stream
	fn unflatten(_: &[u8]) -> Result<T>;

	// TODO: The Haiku API also implements AllowsTypeCode() for each supported
	// type to for example support unflattening a mime type also as a string
	// type. For now this is not implemented here, as these inferences can be
	// made in the code that uses the API to unflatten.
}

impl Flattenable<bool> for bool {
	fn type_code() -> u32 {
		B_BOOL_TYPE
	}

	fn is_fixed_size() -> bool {
		true
	}

	fn flattened_size(&self) -> usize {
		1
	}

	fn flatten(&self) -> Vec<u8> {
		if *self {
			vec![1 as u8]
		} else {
			vec![0 as u8]
		}
	}

	fn unflatten(buffer: &[u8]) -> Result<bool> {
		if buffer.len() != 1 {
			Err(HaikuError::from(ErrorKind::InvalidData))
		} else if buffer[0] == 0 {
			Ok(false)
		} else {
			Ok(true)
		}
	}
}

impl Flattenable<i8> for i8 {
	fn type_code() -> u32 {
		B_INT8_TYPE
	}

	fn is_fixed_size() -> bool {
		true
	}

	fn flattened_size(&self) -> usize {
		1
	}

	fn flatten(&self) -> Vec<u8> {
		vec![*self as u8]
	}

	fn unflatten(buffer: &[u8]) -> Result<i8> {
		if buffer.len() != 1 {
			Err(HaikuError::from(ErrorKind::InvalidData))
		} else {
			Ok(buffer[0] as i8)
		}
	}
}

impl Flattenable<i16> for i16 {
	fn type_code() -> u32 {
		B_INT16_TYPE
	}

	fn flattened_size(&self) -> usize {
		2
	}

	fn is_fixed_size() -> bool {
		true
	}

	fn flatten(&self) -> Vec<u8> {
		let data = self.to_ne_bytes();
		data.to_vec()
	}

	fn unflatten(buffer: &[u8]) -> Result<i16> {
		if buffer.len() != 2 {
			Err(HaikuError::from(ErrorKind::InvalidData))
		} else {
			Ok(buffer.iter().rev().fold(0, |acc, &b| (acc << 8) | b as i16))
		}
	}
}

impl Flattenable<i32> for i32 {
	fn type_code() -> u32 {
		B_INT32_TYPE
	}

	fn flattened_size(&self) -> usize {
		4
	}

	fn is_fixed_size() -> bool {
		true
	}

	fn flatten(&self) -> Vec<u8> {
		let data = self.to_ne_bytes();
		data.to_vec()
	}

	fn unflatten(buffer: &[u8]) -> Result<i32> {
		if buffer.len() != 4 {
			Err(HaikuError::from(ErrorKind::InvalidData))
		} else {
			Ok(buffer.iter().rev().fold(0, |acc, &b| (acc << 8) | b as i32))
		}
	}
}

impl Flattenable<i64> for i64 {
	fn type_code() -> u32 {
		B_INT64_TYPE
	}

	fn flattened_size(&self) -> usize {
		8
	}

	fn is_fixed_size() -> bool {
		true
	}

	fn flatten(&self) -> Vec<u8> {
		let data = self.to_ne_bytes();
		data.to_vec()
	}

	fn unflatten(buffer: &[u8]) -> Result<i64> {
		if buffer.len() != 8 {
			Err(HaikuError::from(ErrorKind::InvalidData))
		} else {
			Ok(buffer.iter().rev().fold(0, |acc, &b| (acc << 8) | b as i64))
		}
	}
}

impl Flattenable<u8> for u8 {
	fn type_code() -> u32 {
		B_UINT8_TYPE
	}

	fn is_fixed_size() -> bool {
		true
	}

	fn flattened_size(&self) -> usize {
		1
	}

	fn flatten(&self) -> Vec<u8> {
		vec![*self]
	}

	fn unflatten(buffer: &[u8]) -> Result<u8> {
		if buffer.len() != 1 {
			Err(HaikuError::from(ErrorKind::InvalidData))
		} else {
			Ok(buffer[0])
		}
	}
}

impl Flattenable<u16> for u16 {
	fn type_code() -> u32 {
		B_UINT16_TYPE
	}

	fn flattened_size(&self) -> usize {
		2
	}

	fn is_fixed_size() -> bool {
		true
	}

	fn flatten(&self) -> Vec<u8> {
		let data = self.to_ne_bytes();
		data.to_vec()
	}

	fn unflatten(buffer: &[u8]) -> Result<u16> {
		if buffer.len() != 2 {
			Err(HaikuError::from(ErrorKind::InvalidData))
		} else {
			Ok(buffer.iter().rev().fold(0, |acc, &b| (acc << 8) | b as u16))
		}
	}
}

impl Flattenable<u32> for u32 {
	fn type_code() -> u32 {
		B_UINT32_TYPE
	}

	fn flattened_size(&self) -> usize {
		4
	}

	fn is_fixed_size() -> bool {
		true
	}

	fn flatten(&self) -> Vec<u8> {
		let data = self.to_ne_bytes();
		data.to_vec()
	}

	fn unflatten(buffer: &[u8]) -> Result<u32> {
		if buffer.len() != 4 {
			Err(HaikuError::from(ErrorKind::InvalidData))
		} else {
			Ok(buffer.iter().rev().fold(0, |acc, &b| (acc << 8) | b as u32))
		}
	}
}

impl Flattenable<u64> for u64 {
	fn type_code() -> u32 {
		B_UINT64_TYPE
	}

	fn flattened_size(&self) -> usize {
		8
	}

	fn is_fixed_size() -> bool {
		true
	}

	fn flatten(&self) -> Vec<u8> {
		let data = self.to_ne_bytes();
		data.to_vec()
	}

	fn unflatten(buffer: &[u8]) -> Result<u64> {
		if buffer.len() != 8 {
			Err(HaikuError::from(ErrorKind::InvalidData))
		} else {
			Ok(buffer.iter().rev().fold(0, |acc, &b| (acc << 8) | b as u64))
		}
	}
}

impl Flattenable<f32> for f32 {
	fn type_code() -> u32 {
		B_FLOAT_TYPE
	}

	fn flattened_size(&self) -> usize {
		4
	}

	fn is_fixed_size() -> bool {
		true
	}

	fn flatten(&self) -> Vec<u8> {
		let data = self.to_ne_bytes();
		data.to_vec()
	}

	fn unflatten(buffer: &[u8]) -> Result<f32> {
		if buffer.len() != 4 {
			Err(HaikuError::from(ErrorKind::InvalidData))
		} else {
			let tmp: u32 = buffer.iter().rev().fold(0, |acc, &b| (acc << 8) | b as u32);
			let tmp: f32 = f32::from_bits(tmp);
			Ok(tmp)
		}
	}
}

impl Flattenable<f64> for f64 {
	fn type_code() -> u32 {
		B_DOUBLE_TYPE
	}

	fn flattened_size(&self) -> usize {
		8
	}

	fn is_fixed_size() -> bool {
		true
	}

	fn flatten(&self) -> Vec<u8> {
		let data = self.to_ne_bytes();
		data.to_vec()
	}

	fn unflatten(buffer: &[u8]) -> Result<f64> {
		if buffer.len() != 8 {
			Err(HaikuError::from(ErrorKind::InvalidData))
		} else {
			let tmp: u64 = buffer.iter().rev().fold(0, |acc, &b| (acc << 8) | b as u64);
			let tmp: f64 = f64::from_bits(tmp);
			Ok(tmp)
		}
	}
}

impl Flattenable<String> for String {
	fn type_code() -> u32 {
		B_STRING_TYPE
	}

	fn flattened_size(&self) -> usize {
		self.as_bytes().len() + 1 // The C-String will have an additional \0
	}

	fn is_fixed_size() -> bool {
		false
	}

	fn flatten(&self) -> Vec<u8> {
		let data = CString::new(self.clone()).unwrap();
		data.into_bytes_with_nul()
	}

	fn unflatten(buffer: &[u8]) -> Result<String> {
		let s = match CStr::from_bytes_with_nul(buffer) {
			Ok(s) => s,
			Err(e) => return Err(HaikuError::new(ErrorKind::InvalidData, format!("{}", e))),
		};
		let s_vec = s.to_bytes().to_vec();
		match String::from_utf8(s_vec) {
			Ok(s) => Ok(s),
			Err(_) => Err(HaikuError::new(
				ErrorKind::InvalidData,
				"Invalid UTF8 characters",
			)),
		}
	}
}

#[test]
fn test_flattenable_primitives() {
	let value: u8 = 150;
	let flattened_value = value.flatten();
	assert_eq!(flattened_value.len(), value.flattened_size());
	assert_eq!(value, flattened_value[0]);

	let value: i64 = -3_223_372_036_854_775_807;
	let flattened_value = value.flatten();
	let unflattened_value = i64::unflatten(&flattened_value).unwrap();
	assert_eq!(value, unflattened_value);

	let value = "This is a test string".to_string();
	let flattened_value = value.flatten();
	let unflattened_value = String::unflatten(&flattened_value).unwrap();
	assert_eq!(value, unflattened_value);
}
#[test]
fn test_flattenable_roundtrip() {
	macro_rules! roundtrip {
		($t:ty, $value:expr) => {{
			let value: $t = $value;
			let flat = value.flatten();
			assert_eq!(flat.len(), value.flattened_size(), stringify!($t));
			assert_eq!(<$t>::unflatten(&flat).unwrap(), value, stringify!($t));
		}};
	}

	roundtrip!(bool, true);
	roundtrip!(bool, false);
	roundtrip!(i8, i8::MIN);
	roundtrip!(i8, i8::MAX);
	roundtrip!(i16, i16::MIN);
	roundtrip!(i16, i16::MAX);
	roundtrip!(i32, i32::MIN);
	roundtrip!(i32, i32::MAX);
	roundtrip!(i64, i64::MIN);
	roundtrip!(i64, i64::MAX);
	roundtrip!(u8, u8::MIN);
	roundtrip!(u8, u8::MAX);
	roundtrip!(u16, u16::MIN);
	roundtrip!(u16, u16::MAX);
	roundtrip!(u32, u32::MIN);
	roundtrip!(u32, u32::MAX);
	roundtrip!(u64, u64::MIN);
	roundtrip!(u64, u64::MAX);
	roundtrip!(f32, 0.0);
	roundtrip!(f32, 1.5);
	roundtrip!(f32, f32::MIN);
	roundtrip!(f32, f32::MAX);
	roundtrip!(f64, 0.0);
	roundtrip!(f64, 1.5);
	roundtrip!(f64, f64::MIN);
	roundtrip!(f64, f64::MAX);
	roundtrip!(String, String::new());
	roundtrip!(String, String::from("This is a test string"));
}

#[test]
fn test_flattenable_fixed_size() {
	assert!(bool::is_fixed_size());
	assert!(i8::is_fixed_size());
	assert!(i16::is_fixed_size());
	assert!(i32::is_fixed_size());
	assert!(i64::is_fixed_size());
	assert!(u8::is_fixed_size());
	assert!(u16::is_fixed_size());
	assert!(u32::is_fixed_size());
	assert!(u64::is_fixed_size());
	assert!(f32::is_fixed_size());
	assert!(f64::is_fixed_size());
	assert!(!String::is_fixed_size());
}

#[test]
fn test_flattenable_float_bits() {
	for bits in [
		0x0000_0000_u32,
		0x0000_0001,
		0x3f80_0000,
		0x7fc0_0000,
		0x8000_0000,
	] {
		let value = f32::from_bits(bits);
		let unflattened = f32::unflatten(&value.flatten()).unwrap();
		assert_eq!(unflattened.to_bits(), bits);
	}

	for bits in [
		0x0000_0000_0000_0000_u64,
		0x0000_0000_0000_0001,
		0x3ff0_0000_0000_0000,
		0x7ff8_0000_0000_0000,
		0x8000_0000_0000_0000,
	] {
		let value = f64::from_bits(bits);
		let unflattened = f64::unflatten(&value.flatten()).unwrap();
		assert_eq!(unflattened.to_bits(), bits);
	}
}

#[cfg(target_endian = "little")]
#[test]
fn test_flattenable_byte_order() {
	assert_eq!(1_i16.flatten(), vec![1, 0]);
	assert_eq!(0x1234_u16.flatten(), vec![0x34, 0x12]);
	assert_eq!(0x1234_i32.flatten(), vec![0x34, 0x12, 0, 0]);
	assert_eq!(0xdead_beef_u32.flatten(), vec![0xef, 0xbe, 0xad, 0xde]);
	assert_eq!(
		0x0123_4567_89ab_cdef_u64.flatten(),
		vec![0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01]
	);
	assert_eq!(
		f32::from_bits(0x3f80_0000).flatten(),
		vec![0, 0, 0x80, 0x3f]
	);
	assert_eq!(
		f64::from_bits(0x3ff0_0000_0000_0000).flatten(),
		vec![0, 0, 0, 0, 0, 0, 0xf0, 0x3f]
	);
}

#[test]
fn test_flattenable_invalid_data() {
	assert!(matches!(
		i16::unflatten(&[0, 0, 0]).unwrap_err().kind(),
		ErrorKind::InvalidData
	));
	assert!(matches!(
		i32::unflatten(&[0]).unwrap_err().kind(),
		ErrorKind::InvalidData
	));
	assert!(matches!(
		f32::unflatten(&[0, 0]).unwrap_err().kind(),
		ErrorKind::InvalidData
	));
	assert!(matches!(
		String::unflatten(&[]).unwrap_err().kind(),
		ErrorKind::InvalidData
	));
}
