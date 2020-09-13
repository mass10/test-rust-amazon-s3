#[allow(unused_imports)]
use core::marker::Sync;
#[allow(unused_imports)]
use std::future::Future;

extern crate rusoto_core;
extern crate rusoto_s3;

mod lib;

fn get_s3_client() -> rusoto_s3::S3Client {
	return rusoto_s3::S3Client::new(rusoto_core::Region::UsEast1);
}

async fn list_objects(s3: &dyn rusoto_s3::S3) {
	let request = rusoto_s3::ListObjectsRequest::default();
	let result = s3.list_objects(request);
	let result = result.await;
	if result.is_err() {
		println!("[ERROR] {:?}", result.err().unwrap());
		return;
	}
	let result = result.ok().unwrap();
	println!("{:?}", result);
}

#[allow(unreachable_code)]
#[allow(unused_must_use)]
async fn put_object_example() -> std::result::Result<(), Box<dyn std::error::Error>> {
	// セットアップ
	let user_profile_dir = std::env::var("USERPROFILE")?;
	println!("{}", user_profile_dir);

	for (key, value) in std::env::vars() {
		println!("{}: {}", key, value);
	}
	return Ok(());
	std::env::set_var("AWS_ACCESS_KEY_ID", "xxxxx");
	std::env::set_var("AWS_SECRET_ACCESS_KEY", "xxxxx");

	// クライアントの作成
	let mut client = get_s3_client();
	let mut _request = rusoto_s3::PutObjectRequest::default();

	// initialize object
	let data = r#"{
        "name": "ジミ ヘンドリックス",
        "age": 28,
        "address": "東京都練馬区練馬1-1-1"
    }"#;
	let value: serde_json::Value = serde_json::from_str(&data).unwrap();

	let mut request = rusoto_s3::PutObjectRequest::default();
	// request.bucket = String::from("d.techtouch.jp");
	request.bucket = String::from("my-bucket-20200901");
	request.key = String::from("/tmp/dummy.json");
	request.body = Some(format!("{}", value).into_bytes().into());

	list_objects(&client);

	// client.
	// request.await;
	// request.await;
	// client.into();
	// client.sync(request).sync().unwrap();

	// client.

	// アップロード
	// let _result = client.put_object(request).sync().unwrap();

	println!("Hello, world!");

	return Ok(());
}

fn main() {
	let conf = lib::configuration::Configuration {};
	let result = conf.configure();
	if result.is_err() {
		let err = result.err();
		println!("[ERROR] {:?}", err);
		return;
	}
	let _result = async {
		let _result = put_object_example().await;
		let _result = _result.unwrap();
		println!("{:?}", _result);
	};
}
