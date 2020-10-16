Rust Apache httpd module
========================


Clone repository
----------------
```bash
git clone https://github.com/studersi/rust-test-apache.git
cd rust-test-apache
```



Apache httpd src
----------------
* Download Apache httpd
```bash
wget -P downloads/ https://downloads.apache.org/httpd/httpd-2.4.46.tar.gz
tar -xvf downloads/httpd-2.4.46.tar.gz -C downloads/
```
* Download APR
```bash
wget -P downloads/ https://downloads.apache.org/apr/apr-1.7.0.tar.gz
tar -xvf downloads/apr-1.7.0.tar.gz -C downloads/
```



Docker
------
* Build container
```bash
docker build -t rust-apache .
```
* Run container
```bash
docker run -it -v "$(pwd):/home/rustacian" rust-apache
```



C example
---------
* Build module
```bash
apxs -a -c mod_example.c
```



Rust example
------------
* Update `wrapper.h`
```bash
grep "#include.*.h" mod_example.c | sed 's/[<>]/"/g' > mod_rs/wrapper.h
```
* Build Rust project
```bash
cd mod_rs
cargo build
```
