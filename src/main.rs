use gethostname::gethostname;
use std::env;

fn main() {
    match ureq::put("http://169.254.169.254/latest/api/token").set("X-aws-ec2-metadata-token-ttl-seconds", "30").call() {
        Ok(response) => {
            let token = response.into_string().unwrap();
            match ureq::get("http://169.254.169.254/latest/meta-data/ami-id").set("X-aws-ec2-metadata-token", &token).call() {
                Ok(response) => {
                    authenticated(response.into_string().unwrap());
                },
                Err(_) => {
                    authenticated(gethostname().into_string().unwrap());
                }
            }
        },
        Err(_) => {
            authenticated(gethostname().into_string().unwrap());
        }
    }
}

fn authenticated(amiid: String) {

    println!("amiid: {amiid}");

    // https://doc.rust-lang.org/std/env/consts/constant.OS.html

    if cfg!(target_os = "windows") {

        println!("{}", env::consts::OS);

    } else { // linux distribution

        println!("{}", env::consts::OS);

    }
}