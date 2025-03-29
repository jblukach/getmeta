use std::io::Write;

pub fn collectmsft(amiid: String) -> String {
    let local = std::env::current_dir().unwrap();
    let path = format!("{}\\mmi-{}.csv", local.display(), &amiid);
    let mut file = std::fs::File::create(&path).unwrap();
    writeln!(file, "amiid,fpath,fname,fsize,b3hash,b3name,b3path,b3dir").unwrap();
    for entry in walkdir::WalkDir::new("c:\\").into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {              
            let fname = entry.file_name().to_str().unwrap();
            let mut b3hash;
            let fsize: String;
            let test = std::fs::File::open(entry.path());
            if test.is_err() {
                println!(" - Denied: {}", entry.path().display().to_string());
                b3hash = "DENIED".to_string();
                fsize = "0".to_string();
            } else {
                let metadata = std::fs::metadata(entry.path()).unwrap();
                fsize = metadata.len().to_string();
                if fsize == "0" {
                    b3hash = "ZERO".to_string();
                } else if fsize.parse::<u64>().unwrap() > 10*104857599 { // 1GB
                    println!(" - Large: {}", entry.path().display().to_string());
                    b3hash = "LARGE".to_string();
                } else {
                    b3hash = mmi::b3content(entry.path());
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
            let b3name = mmi::b3text(fname.to_string());
            let fpath = entry.path().display().to_string();
            let b3path = mmi::b3windows(fpath);
            let fdir = entry.path().parent().unwrap();
            let b3dir = mmi::b3windows(fdir.display().to_string());
            let fpath = entry.path().display().to_string();
            let fname = fname.replace(",", "|||");
            let fpath = fpath.replace(",", "|||");
            writeln!(file, "{},{},{},{},{},{},{},{}", &amiid, &fpath, &fname, &fsize, &b3hash, &b3name, &b3path, &b3dir).unwrap();
        }
    }
    file.sync_all().unwrap();
    return path;
}

pub fn collectunix(amiid: String) -> String {
    let local = std::env::current_dir().unwrap();
    let path = format!("{}/mmi-{}.csv", local.display(), &amiid);
    let mut file = std::fs::File::create(&path).unwrap();
    writeln!(file, "amiid,fpath,fname,fsize,b3hash,b3name,b3path,b3dir").unwrap();
    for entry in walkdir::WalkDir::new("/").into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() { 
            let fname = entry.file_name().to_str().unwrap();
            let mut b3hash;
            let fsize: String;
            let test = std::fs::File::open(entry.path());
            if test.is_err() {
                println!(" - Denied: {}", entry.path().display().to_string());
                b3hash = "DENIED".to_string();
                fsize = "0".to_string();
            } else {
                let metadata = std::fs::metadata(entry.path()).unwrap();
                fsize = metadata.len().to_string();
                if fsize == "0" {
                    b3hash = "ZERO".to_string();
                } else if fsize.parse::<u64>().unwrap() > 10*104857599 { // 1GB
                    println!(" - Large: {}", entry.path().display().to_string());
                    b3hash = "LARGE".to_string();
                } else {
                    b3hash = mmi::b3content(entry.path());
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
            let b3name = mmi::b3text(fname.to_string());
            let fpath = entry.path().display().to_string();
            let b3path = mmi::b3unix(fpath);
            let fdir = entry.path().parent().unwrap();
            let b3dir = mmi::b3unix(fdir.display().to_string());
            let fpath = entry.path().display().to_string();
            let fname = fname.replace(",", "|||");
            let fpath = fpath.replace(",", "|||");
            writeln!(file, "{},{},{},{},{},{},{},{}", &amiid, &fpath, &fname, &fsize, &b3hash, &b3name, &b3path, &b3dir).unwrap();
        }
    }
    file.sync_all().unwrap();
    return path;
}

pub fn hostname() -> String {
    match ureq::put("http://169.254.169.254/latest/api/token").header("X-aws-ec2-metadata-token-ttl-seconds", "30").send("") {
        Ok(mut response) => {
            let token = response.body_mut().read_to_string().unwrap();
            match ureq::get("http://169.254.169.254/latest/meta-data/ami-id").header("X-aws-ec2-metadata-token", &token).call() {
                Ok(mut response) => {
                    return response.body_mut().read_to_string().unwrap();
                },
                Err(_) => {
                    return gethostname::gethostname().into_string().unwrap()
                }
            }
        },
        Err(_) => {
            return gethostname::gethostname().into_string().unwrap();
        }
    }
}