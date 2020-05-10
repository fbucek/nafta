extern crate tempfile;
use std::io::prelude::*;

/// This test is for investigating problem with not deleting dir on GitHub Actions Windows Server
/// https://github.com/fbucek/nafta/issues/9
/// Is it problem with `nafta` or `tmpfile` or GitHub Actions 
/// Info: https://docs.rs/tempfile/3.1.0/tempfile/struct.TempDir.html#resource-leaking
#[test]
fn test_tempdir() {
    let tmp_dir = tempfile::Builder::new()
    .prefix(env!("CARGO_PKG_NAME"))
    .rand_bytes(5)
    .tempdir()
    .expect("not possible to create tempfile");
    // let tmp_dir = Box::new(tmp_dir);

    let path_dir = tmp_dir.path().to_owned();
    let path_file = tmp_dir.path().join("file.txt");

    let mut buffer = std::fs::File::create(&path_file)
        .expect(format!("Not possible to open file {:?}", &path_file).as_str());

    buffer.write_all(b"hello")
        .expect(format!("Not possible to write 'hello' to file: {:?}", &path_file).as_str());
    
    drop(buffer);
    drop(tmp_dir);
    assert!(!path_file.exists());
    assert!(!path_dir.exists());
}
