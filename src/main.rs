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
	/// コンフィギュレーション
	pub fn configure() -> Result<MyS3ClientImpl, Box<dyn std::error::Error>> {
		let client = rusoto_s3::S3Client::new(rusoto_core::Region::ApNortheast1);
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
		return Ok(instance);
	}

	fn get_s3_client(&self) -> &rusoto_s3::S3Client {
		return &self.client;
	}

	fn get_s3(&mut self) -> &dyn rusoto_s3::S3 {
		let client = self.get_s3_client();
		return client;
	}

	/// オブジェクトを列挙します。
	fn list_objects_example(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
		let bucket_name = "my-bucket-20200901";
		let s3: &dyn rusoto_s3::S3 = self.get_s3();

		println!("[TRACE] オブジェクトを列挙しています...");
		let mut request = rusoto_s3::ListObjectsRequest::default();
		request.bucket = "my-bucket-20200901".to_string();
		request.bucket = bucket_name.to_string();
		// request.key = "dummy.json".to_string();
		// request.bucket = "".to_string();
		let result = s3.list_objects(request);
		let result = tokio::runtime::Runtime::new().expect("ERROR").block_on(result);
		if result.is_err() {
			println!("[ERROR] {:?}", result.err().unwrap());
			return Ok(());
		}
		let result = result.ok().unwrap();
		// アイテムの取り出し方がわからない。単純に for e in result {} できないようだ。
		println!("{:?}", result);

		return Ok(());
	}

	/// ？？？
	fn list_parts_example(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
		let bucket_name = "my-bucket-20200901";
		let s3: &dyn rusoto_s3::S3 = self.get_s3();

		println!("[TRACE] オブジェクトを列挙しています...");

		let mut request = rusoto_s3::ListPartsRequest::default();
		request.bucket = "my-bucket-20200901".to_string();
		request.bucket = bucket_name.to_string();
		let result = s3.list_parts(request);
		let result = tokio::runtime::Runtime::new().expect("ERROR").block_on(result)?;
		for e in result.parts {
			println!("[TRACE] {:?}", e);
		}
		// for e in result {}
		// let key_count = result.key_count.unwrap();
		// let item: rusoto_s3::listobject = result;
		// let item = result[0];
		// for e in result {
		// 	println!("[TRACE] {:?}", e);
		// }

		return Ok(());
	}

	/// オブジェクトを送信
	fn put_object(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
		println!("[TRACE] オブジェクトを作成");

		// リクエストオブジェクトを作成
		let data = r#"{
        "name": "ジミ ヘンドリックス",
        "age": 28,
        "address": "東京都練馬区練馬1-1-1"
    }"#;
		let value: serde_json::Value = serde_json::from_str(&data).unwrap();
		// let value = format!("{}", value);
		let value = format!("{}", value);

		let mut request = rusoto_s3::PutObjectRequest::default();
		// request.content_type = Some(String::from("application/json"));
		request.bucket = "my-bucket-20200901".to_string();
		request.key = "dummy.json".to_string();
		request.body = Some(value.into_bytes().into());

		// リクエスト
		println!("[TRACE] S3 PUT");
		let s3 = self.get_s3();
		let result = s3.put_object(request);
		let result = tokio::runtime::Runtime::new().expect("ERROR").block_on(result);
		if result.is_err() {
			println!("[ERROR] オブジェクトの保存に失敗しました。理由: {:?}", result.err().unwrap());
			return Ok(());
		}
		let result = result.ok().unwrap();
		println!("{:?}", result);
		return Ok(());
	}
}

/// エントリーポイント
// #[tokio::main]
fn main() {
	// コンフィギュレーションと初期化
	let result = MyS3ClientImpl::configure();
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		return;
	}

	let mut client = result.ok().unwrap();

	// Amazon S3 へオブジェクトを作成します。
	let result = client.put_object();
	if result.is_err() {
		println!("[TRACE] <main()> {}", result.err().unwrap());
	}

	let result = client.list_objects_example();
	if result.is_err() {
		println!("[TRACE] <main()> {}", result.err().unwrap());
	}

	let result = client.list_parts_example();
	if result.is_err() {
		println!("[TRACE] <main()> {}", result.err().unwrap());
	}

	println!("[TRACE] Ok.");
}
