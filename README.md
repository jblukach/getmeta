# getmeta

### Installation

```
curl https://sh.rustup.rs -sSf | sh -s -- -y
source "$HOME/.cargo/env"
```

### Development

```
cargo build
cargo run
```

### Artifacts

- amiid
- fpath
- fname
- fsize
- b3hash
- b3name
- b3path
- b3dir

### Exclusion

- File paths containing **,** a.k.a. comma are excluded!

### Classifications

- **DENIED** Permission Issue
- **EMPTY** Empty File Hash
- **ERROR** Content Hash Error
- **LARGE** File Size 1+ GB
- **ZERO** Zero File Size

### Requirements

```
yum install gcc git openssl-devel -y
```

### Binary Build

```
cargo build --release
```

![Meta Information](images/matchmeta.png)