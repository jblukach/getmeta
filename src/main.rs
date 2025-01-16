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

fn output(amiid: String) {
    collection(amiid);
}

fn collection(amiid: String) {
    if cfg!(target_os = "windows") {
        let local = std::env::current_dir().unwrap();
        println!("Output: {}\\mmi-csv-{}.csv", local.display(), &amiid);
        let path = format!("{}\\mmi-csv-{}.csv", local.display(), &amiid);
        let mut file = std::fs::File::create(&path).unwrap();
        writeln!(file, "amiid,fpath,fname,fsize,b3hash,b3name,b3path,b3dir").unwrap();
        for entry in walkdir::WalkDir::new("c:\\").into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() { 
                if entry.path().display().to_string().contains(",") {
                    println!(" - Skipped: {}", entry.path().display().to_string());
                } else {               
                    let fname = entry.file_name().to_str().unwrap();
                    let metadata = std::fs::metadata(entry.path()).unwrap();
                    let fsize = metadata.len().to_string();
                    let b3hash = b3content(entry.path());
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
        let out = format!("{}\\mmi-pqt-{}.parquet", local.display(), &amiid);
        let file = std::fs::File::create(out).unwrap();
        let mut df = CsvReadOptions::default().with_has_header(true).try_into_reader_with_file_path(Some(path.into())).unwrap().finish().unwrap();
        ParquetWriter::new(file).with_compression(ParquetCompression::Snappy).finish(&mut df).unwrap();
        println!("Done: {}\\mmi-pqt-{}.parquet", local.display(), &amiid);
    } else {
        let local = std::env::current_dir().unwrap();
        println!("Output: {}/mmi-csv-{}.csv", local.display(), &amiid);
        let path = format!("{}/mmi-csv-{}.csv", local.display(), &amiid);
        let mut file = std::fs::File::create(&path).unwrap();
        writeln!(file, "amiid,fpath,fname,fsize,b3hash,b3name,b3path,b3dir").unwrap();
        for entry in walkdir::WalkDir::new("/workspaces/getmeta").into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() { 
                if entry.path().display().to_string().contains(",") {
                    println!(" - Skipped: {}", entry.path().display().to_string());
                } else {               
                    let fname = entry.file_name().to_str().unwrap();
                    let metadata = std::fs::metadata(entry.path()).unwrap();
                    let fsize = metadata.len().to_string();
                    let b3hash = b3content(entry.path());
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
        let out = format!("{}/mmi-pqt-{}.parquet", local.display(), &amiid);
        let file = std::fs::File::create(out).unwrap();
        let mut df = CsvReadOptions::default().with_has_header(true).try_into_reader_with_file_path(Some(path.into())).unwrap().finish().unwrap();
        ParquetWriter::new(file).with_compression(ParquetCompression::Snappy).finish(&mut df).unwrap();
        println!("Done: {}/mmi-pqt-{}.parquet", local.display(), &amiid);
    }
}

fn b3content(path: &std::path::Path) -> String {
    let mut file = std::fs::File::open(path).unwrap();
    let mut hasher = blake3::Hasher::new();
    std::io::copy(&mut file, &mut hasher).unwrap();
    let hash = hasher.finalize();
    return hash.to_string()
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
    if out[1] == "home" {
        out[2] = "user";
        let path = out.join("/");
        let hash = b3text(path);
        return hash.to_string();
    } else if out[1] == "Users" && out[2] != "Shared"  {
        out[2] = "user";
        let path = out.join("/");
        let hash = b3text(path);
        return hash.to_string();
    } else {
        let hash = b3text(path);
        return hash.to_string();
    }
}

fn b3windows(path: String) -> String {
    let out = path.split('\\');
    let mut out = out.collect::<Vec<&str>>();
    if out[1] == "Users" && (out[2] != "Default" || out[2] != "Public") {
        out[2] = "user";
        let path = out.join("/");
        let hash = b3text(path);
        return hash.to_string();
    } else {
        let hash = b3text(path);
        return hash.to_string();
    }
}