mod external {
	extern "C" {
		pub fn log_utf8(len: u64, ptr: u64);
	}
}

mod api {
	use crate::external;
	pub fn log(msg: &[u8]) {
		unsafe {
			external::log_utf8(msg.len() as u64, msg.as_ptr() as u64);
		}
	}
}

#[no_mangle]
pub fn log_hello() {
	api::log(b"Hello, world!");
}
