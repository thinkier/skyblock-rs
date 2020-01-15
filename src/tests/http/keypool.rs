use crate::http::Key;
use std::time::{SystemTime, Duration};
use std::alloc::System;
use std::thread;

#[test]
fn disallow() {
	let mut key = Key::new("x", 120, 60);

	for _ in 0..120 {
		key.consume();
	}

	assert_eq!(key.consume(), None);
}

#[test]
fn wait() {
	let start = SystemTime::now();
	let mut key = Key::new("x", 120, 1);

	for _ in 0..=120 {
		loop {
			if key.consume().is_some() {
				break;
			} else {
				thread::sleep(Duration::from_millis(50));
			}
		}
	}

	assert_eq!(start.elapsed().unwrap().as_secs(), 1);
}