//! Cache auth information

// pub fn get_auth_token() -> Option<AccessTokenResponse> {
// 	let mut cache = CACHE.lock().unwrap();
// 	if cache.auth_token.is_none() {
// 		return None;
// 	}
// 	let auth_token = cache.auth_token.as_ref().unwrap();
// 	if auth_token.expires_in < SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() {
// 		return None;
// 	}
// 	Some(auth_token.clone())
// }
