#[allow(unused_imports)]
use core::marker::Sync;
#[allow(unused_imports)]
use std::future::Future;

extern crate rusoto_core;
extern crate rusoto_s3;

mod lib;

#[derive(Debug, Clone)]
pub struct MyStringError {
	pub message: String,
	// pub line: usize,
	// pub column: usize,
}

impl std::fmt::Display for MyStringError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
		write!(f, "{}", self.message)
	}
}

impl std::error::Error for MyStringError {
	fn description(&self) -> &str {
		&self.message
	}
}

struct MyS3ClientImpl {
	client: Box<rusoto_s3::S3Client>,
}

impl MyS3ClientImpl {
	pub fn configure() -> Result<MyS3ClientImpl, Box<dyn std::error::Error>> {
		let client = rusoto_s3::S3Client::new(rusoto_core::Region::UsEast1);
		let instance = MyS3ClientImpl { client: Box::new(client) };
		let conf = lib::configuration::Configuration {};
		let result = conf.configure();
		if result.is_err() {
			let err = result.err();
			println!("[ERROR] {:?}", err);
			return Err(Box::new(MyStringError {
				message: "続行不能なエラーです。".to_string(),
			}));
		}
		// self.client = rusoto_s3::S3Client::new(rusoto_core::Region::UsEast1);
		return Ok(instance);
	}

	fn get_s3_client(&self) -> &rusoto_s3::S3Client {
		return &self.client;
		// return rusoto_s3::S3Client::new(rusoto_core::Region::UsEast1);
	}

	/// オブジェクトを列挙
	async fn list_objects(&self, s3: &dyn rusoto_s3::S3) {
		println!("[TRACE] オブジェクトを列挙");
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

	/// オブジェクトを送信
	async fn put_object(s3: &dyn rusoto_s3::S3) {
		println!("[TRACE] オブジェクトを作成");

		// リクエストオブジェクトを作成
		let data = r#"{
        "name": "ジミ ヘンドリックス",
        "age": 28,
        "address": "東京都練馬区練馬1-1-1"
    }"#;
		let value: serde_json::Value = serde_json::from_str(&data).unwrap();

		let mut request = rusoto_s3::PutObjectRequest::default();
		request.bucket = String::from("my-bucket-20200901");
		request.key = String::from("/tmp/dummy.json");
		request.body = Some(format!("{}", value).into_bytes().into());

		// リクエスト
		let result = s3.put_object(request);
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
	pub async fn put_object_example(&self) -> std::result::Result<(), Box<dyn std::error::Error>> {
		// クライアントの作成
		let client = self.get_s3_client();

		// リクエストオブジェクトを作成
		let data = r#"{
        "name": "ジミ ヘンドリックス",
        "age": 28,
        "address": "東京都練馬区練馬1-1-1"
    }"#;
		let value: serde_json::Value = serde_json::from_str(&data).unwrap();

		let mut request = rusoto_s3::PutObjectRequest::default();
		request.bucket = String::from("my-bucket-20200901");
		request.key = String::from("/tmp/dummy.json");
		request.body = Some(format!("{}", value).into_bytes().into());

		// 列挙
		if false {
			// let client = client as dyn rusoto_s3::S3;
			self.list_objects(&client).await;
		}

		// アップロード
		if true {
			let _result = self.put_object(&client).await;
		}

		println!("Hello, world!");

		return Ok(());
	}
}

/// エントリーポイント
fn main() {
	let s3 = MyS3ClientImpl::configure();
	if !s3.configure() {
		return;
	}
	{
		let future = s3.put_object_example();
		// async_std::task::block_on(future);
		let result = futures::executor::block_on(future);
		if result.is_err() {
			println!("[ERROR] {}", result.err().unwrap());
			return;
		}
	}
}
