extern crate libc;
extern crate libsqlite3_sys as ffi;

use std::ffi::{CStr, CString};
use std::ptr;
use std::os::raw::c_char;

pub struct SqliteConnection {
	conn: *mut ffi::sqlite3,
}

impl SqliteConnection {
	pub fn open(path: &str) -> Result<Self, String> {
		let c_path = CString::new(path).unwrap();
		let mut conn: *mut ffi::sqlite3 = ptr::null_mut();
		let result = unsafe { ffi::sqlite3_open(c_path.as_ptr(), &mut conn) };
		
		if result != ffi::SQLITE_OK {
			Err("Failed to open database".to_string())
		} else {
			Ok(SqliteConnection { conn })
		}
	}

	pub fn close(self) -> Result<(), String> {
		let result = unsafe { ffi::sqlite3_close(self.conn) };
		if result != ffi::SQLITE_OK {
			Err("Failed to close database".to_string())
		} else {
			Ok(())
		}
	}

	pub fn query_test(&self) -> Result<Vec<(i32, String)>, String> {
		let sql = CString::new("SELECT number, text FROM test").unwrap();
		let mut stmt: *mut ffi::sqlite3_stmt = std::ptr::null_mut();
		let prepare_result = unsafe {
			ffi::sqlite3_prepare_v2(self.conn, sql.as_ptr(), -1, &mut stmt, std::ptr::null_mut())
		};

		if prepare_result != ffi::SQLITE_OK {
			return Err("Failed to prepare query".to_string());
		}

		let mut rows = Vec::new();
		loop {
			let step_result = unsafe { ffi::sqlite3_step(stmt) };
			if step_result == ffi::SQLITE_ROW {
				let number = unsafe { ffi::sqlite3_column_int(stmt, 0) };
				let text_ptr = unsafe { ffi::sqlite3_column_text(stmt, 1) as *const c_char };
				let text = unsafe {
					CStr::from_ptr(text_ptr)
						.to_string_lossy()
						.into_owned()
					};
				rows.push((number, text));
			} else if step_result == ffi::SQLITE_DONE {
				break;
			} else {
				unsafe { ffi::sqlite3_finalize(stmt) };
				return Err("Failed to step through query".to_string());
			}
		}

		unsafe { ffi::sqlite3_finalize(stmt) };
		Ok(rows)
	}

	pub fn insert_into_test(&self, number: i32, text: &str) -> Result<(), String> {
		let sql = CString::new(format!("INSERT INTO test (number, text) VALUES (?, ?)")).unwrap();
		let mut stmt: *mut ffi::sqlite3_stmt = std::ptr::null_mut();
		let prepare_result = unsafe {
			ffi::sqlite3_prepare_v2(self.conn, sql.as_ptr(), -1, &mut stmt, std::ptr::null_mut())
		};

		if prepare_result != ffi::SQLITE_OK {
			return Err("Failed to prepare insert".to_string());
		}

		let text_cstr = CString::new(text).unwrap();
		unsafe {
			ffi::sqlite3_bind_int(stmt, 1, number);
			ffi::sqlite3_bind_text(stmt, 2, text_cstr.as_ptr(), -1, ffi::SQLITE_TRANSIENT());
		}

		let step_result = unsafe { ffi::sqlite3_step(stmt) };
		if step_result != ffi::SQLITE_DONE {
			unsafe { ffi::sqlite3_finalize(stmt) };
			return Err("Failed to insert row".to_string());
		}

		unsafe { ffi::sqlite3_finalize(stmt) };
		Ok(())
	}
}

impl Drop for SqliteConnection {
	fn drop(&mut self) {
		let _ = unsafe { ffi::sqlite3_close(self.conn) };
	}
}
