use polars::prelude::*;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let amiid = getmeta::hostname();
    println!("Hostname: {}", amiid);
    let csvfile;
    let path: String;
    let local = std::env::current_dir().unwrap();
    if cfg!(target_os = "windows") {
        path = format!("{}\\mmi-{}.parquet", local.display(), &amiid);
        csvfile = getmeta::collectmsft(amiid.clone());
    } else {
        path = format!("{}/mmi-{}.parquet", local.display(), &amiid);
        csvfile = getmeta::collectunix(amiid.clone());
    }
    println!("CSV File: {}", &csvfile);
    let file = std::fs::File::create(&path).unwrap();
    let mut df = CsvReadOptions::default().with_has_header(true).try_into_reader_with_file_path(Some(PathBuf::from(&csvfile))).unwrap().finish().unwrap();
    ParquetWriter::new(file).with_compression(ParquetCompression::Snappy).finish(&mut df).unwrap();
    println!("Parquet File: {}", &path);
    if args.len() == 4 {
        let uuid = uuid::Uuid::new_v4();
        let s3file = format!("{}/mmi-{}-uuid-{}.parquet", args[4], &amiid, uuid);
        println!("Bucket: s3://{}/{}", args[2], s3file);
        let body = aws_sdk_s3::primitives::ByteStream::from_path(std::path::Path::new(&path)).await.unwrap();
        let region = aws_sdk_s3::config::Region::new(args[3].clone());
        let config = aws_config::from_env().region(region).load().await;
        let client = aws_sdk_s3::Client::new(&config);
        let response = client.put_object().bucket(&args[2]).key(&s3file).body(body).send().await.unwrap();
        println!("Response: {:?}", response);
    } else {
        println!("getmeta <bucket> <region> <prefix>");
    }
}