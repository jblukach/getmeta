use polars::prelude::*;
use std::io::Write;

fn main() {
    match ureq::put("http://169.254.169.254/latest/api/token").set("X-aws-ec2-metadata-token-ttl-seconds", "30").call() {
        Ok(response) => {
            let token = response.into_string().unwrap();
            match ureq::get("http://169.254.169.254/latest/meta-data/ami-id").set("X-aws-ec2-metadata-token", &token).call() {
                Ok(response) => {
                    output(response.into_string().unwrap());
                },
                Err(_) => {
                    output(gethostname::gethostname().into_string().unwrap());
                }
            }
        },
        Err(_) => {
            output(gethostname::gethostname().into_string().unwrap());
        }
    }
}

#[tokio::main]
async fn output(amiid: String) {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 3 {
        let bucket = &args[1];
        let region = &args[2];
        let local = std::env::current_dir().unwrap();
        println!("Output: {}/mmi-{}.parquet", local.display(), &amiid);
        println!("Region: {}", &region);
        collection(amiid, bucket.to_string(), region.to_string()).await;
    } else {
        let local = std::env::current_dir().unwrap();
        println!("Output: {}/mmi-{}.csv", local.display(), &amiid);
        let buckt = "LOCAL".to_string();
        let region = "LOCAL".to_string();
        collection(amiid, buckt, region).await;
    }
}

async fn collection(amiid: String, location: String, region: String) {
    if cfg!(target_os = "windows") {
        let local = std::env::current_dir().unwrap();
        let path = format!("{}\\mmi-{}.csv", local.display(), &amiid);
        let mut file = std::fs::File::create(&path).unwrap();
        writeln!(file, "amiid,fpath,fname,fsize,b3hash,b3name,b3path,b3dir").unwrap();
        for entry in walkdir::WalkDir::new("c:\\").into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() { 
                if entry.path().display().to_string().contains(",") {
                    println!(" - Excluded: {}", entry.path().display().to_string());
                } else {               
                    let fname = entry.file_name().to_str().unwrap();
                    let metadata = std::fs::metadata(entry.path()).unwrap();
                    let fsize = metadata.len().to_string();
                    let mut b3hash;
                    if fsize == "0" {
                        b3hash = "ZERO".to_string();
                    } else if fsize.parse::<u64>().unwrap() > 10*104857599 { // 1GB
                        println!(" - Large: {}", entry.path().display().to_string());
                        b3hash = "LARGE".to_string();
                    } else {
                        let test = std::fs::File::open(entry.path());
                        if test.is_err() {
                            println!(" - Denied: {}", entry.path().display().to_string());
                            b3hash = "DENIED".to_string();
                        } else {
                            b3hash = b3content(entry.path());
                            if b3hash == "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262" {
                                b3hash = "EMPTY".to_string();
                            } else if b3hash == "ERROR" {
                                println!(" - Error: {}", entry.path().display().to_string());
                                b3hash = "ERROR".to_string();
                            } else {
                                b3hash = b3hash;
                            }
                        }
                    }
                    let b3name = b3text(fname.to_string());
                    let fpath = entry.path().display().to_string();
                    let b3path = b3windows(fpath);
                    let fdir = entry.path().parent().unwrap();
                    let b3dir = b3windows(fdir.display().to_string());
                    let fpath = entry.path().display().to_string();
                    writeln!(file, "{},{},{},{},{},{},{},{}", &amiid, &fpath, &fname, &fsize, &b3hash, &b3name, &b3path, &b3dir).unwrap();
                }
            }
        }
        file.sync_all().unwrap();
        if &location == "LOCAL" {
            println!("Completed!");
        } else {
            let out = format!("{}\\mmi-{}.parquet", local.display(), &amiid);
            let file = std::fs::File::create(out).unwrap();
            let mut df = CsvReadOptions::default().with_has_header(true).try_into_reader_with_file_path(Some(path.into())).unwrap().finish().unwrap();
            ParquetWriter::new(file).with_compression(ParquetCompression::Snappy).finish(&mut df).unwrap();
            let upload = location.split('/');
            let upload = upload.collect::<Vec<&str>>();
            let uuid = uuid::Uuid::new_v4();
            let s3file = format!("{}/mmi-{}-uuid-{}.parquet", upload[1], &amiid, uuid);
            println!("Bucket: s3://{}/{}", upload[0], s3file);
            let output = format!("{}\\mmi-{}.parquet", local.display(), &amiid);
            let body = aws_sdk_s3::primitives::ByteStream::from_path(std::path::Path::new(&output)).await.unwrap();
            let region = aws_sdk_s3::config::Region::new(region);
            let config = aws_config::from_env().region(region).load().await;
            let client = aws_sdk_s3::Client::new(&config);
            let response = client.put_object().bucket(upload[0]).key(&s3file).body(body).send().await.unwrap();
            println!("Response: {:?}", response);
        }
    } else {
        let local = std::env::current_dir().unwrap();
        let path = format!("{}/mmi-{}.csv", local.display(), &amiid);
        let mut file = std::fs::File::create(&path).unwrap();
        writeln!(file, "amiid,fpath,fname,fsize,b3hash,b3name,b3path,b3dir").unwrap();
        for entry in walkdir::WalkDir::new("/").into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() { 
                if entry.path().display().to_string().contains(",") {
                    println!(" - Excluded: {}", entry.path().display().to_string());
                } else {               
                    let fname = entry.file_name().to_str().unwrap();
                    let metadata = std::fs::metadata(entry.path()).unwrap();
                    let fsize = metadata.len().to_string();
                    let mut b3hash;
                    if fsize == "0" {
                        b3hash = "ZERO".to_string();
                    } else if fsize.parse::<u64>().unwrap() > 10*104857599 { // 1GB
                        println!(" - Large: {}", entry.path().display().to_string());
                        b3hash = "LARGE".to_string();
                    } else {
                        let test = std::fs::File::open(entry.path());
                        if test.is_err() {
                            println!(" - Denied: {}", entry.path().display().to_string());
                            b3hash = "DENIED".to_string();
                        } else {
                            b3hash = b3content(entry.path());
                            if b3hash == "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262" {
                                b3hash = "EMPTY".to_string();
                            } else if b3hash == "ERROR" {
                                println!(" - Error: {}", entry.path().display().to_string());
                                b3hash = "ERROR".to_string();
                            } else {
                                b3hash = b3hash;
                            }
                        }
                    }
                    let b3name = b3text(fname.to_string());
                    let fpath = entry.path().display().to_string();
                    let b3path = b3unix(fpath);
                    let fdir = entry.path().parent().unwrap();
                    let b3dir = b3unix(fdir.display().to_string());
                    let fpath = entry.path().display().to_string();
                    writeln!(file, "{},{},{},{},{},{},{},{}", &amiid, &fpath, &fname, &fsize, &b3hash, &b3name, &b3path, &b3dir).unwrap();
                }
            }
        }
        file.sync_all().unwrap();
        if &location == "LOCAL" {
            println!("Completed!");
        } else {
            let out = format!("{}/mmi-{}.parquet", local.display(), &amiid);
            let file = std::fs::File::create(out).unwrap();
            let mut df = CsvReadOptions::default().with_has_header(true).try_into_reader_with_file_path(Some(path.into())).unwrap().finish().unwrap();
            ParquetWriter::new(file).with_compression(ParquetCompression::Snappy).finish(&mut df).unwrap();
            let upload = location.split('/');
            let upload = upload.collect::<Vec<&str>>();
            let uuid = uuid::Uuid::new_v4();
            let s3file = format!("{}/mmi-{}-uuid-{}.parquet", upload[1], &amiid, uuid);
            println!("Bucket: s3://{}/{}", upload[0], s3file);
            let output = format!("{}/mmi-{}.parquet", local.display(), &amiid);
            let body = aws_sdk_s3::primitives::ByteStream::from_path(std::path::Path::new(&output)).await.unwrap();
            let region = aws_sdk_s3::config::Region::new(region);
            let config = aws_config::from_env().region(region).load().await;
            let client = aws_sdk_s3::Client::new(&config);
            let response = client.put_object().bucket(upload[0]).key(&s3file).body(body).send().await.unwrap();
            println!("Response: {:?}", response);
        }
    }
}

fn b3content(path: &std::path::Path) -> String {
    match std::fs::File::open(path) {
        Ok(mut file) => {
            let mut hasher = blake3::Hasher::new();
            let _temp = std::io::copy(&mut file, &mut hasher);
            let hash = hasher.finalize();
            hash.to_string()
        },
        Err(_) => "ERROR".to_string(),
    }
}

fn b3text(text: String) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(text.as_bytes());
    let hash = hasher.finalize();
    return hash.to_string()
}

fn b3unix(path: String) -> String {
    let out = path.split('/');
    let mut out = out.collect::<Vec<&str>>();
    if out.len() > 3 {
        if out[1] == "home" {
            out[2] = "user";
            let path = out.join("/");
            let hash = b3text(path);
            return hash.to_string();
        } else if out[1] == "Users" && out[2] != "Shared" {
            out[2] = "user";
            let path = out.join("/");
            let hash = b3text(path);
            return hash.to_string();
        } else {
            let hash = b3text(path);
            return hash.to_string();
        }
    } else {
        let hash = b3text(path);
        return hash.to_string();
    }
}

fn b3windows(path: String) -> String {
    let out = path.split('\\');
    let mut out = out.collect::<Vec<&str>>();
    if out.len() > 3 {
        if out[1] == "Users" && (out[2] != "Default" || out[2] != "Public") {
            out[2] = "user";
            let path = out.join("\\");
            let hash = b3text(path);
            return hash.to_string();
        } else {
            let hash = b3text(path);
            return hash.to_string();
        }
    } else {
        let hash = b3text(path);
        return hash.to_string();
    }
}
