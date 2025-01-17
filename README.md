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

### Requirements

```
yum install gcc openssl-devel
```

### Binary Build

```
cargo build --release
```

![Meta Information](images/matchmeta.png)