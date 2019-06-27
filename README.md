# Sassruist
Sassruist is the evil Sass/Scss ampersand buster.

## Usage
```
USAGE:
    sassruist <filepath>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <filepath>    target filepath
```

## Develop
### Test
```sh
cargo test
```

### Build bin
```sh
cargo build
```

### Start Demo
```sh
wasm-pack build
cd ./www
npm install
npm run start
```

### Build and Deploy demo
Execute build script
```sh
sh ./build.sh
```
and push master branch.
