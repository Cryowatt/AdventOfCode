use std::{
    fs::{copy, read_dir},
    path::PathBuf,
};

fn main() -> std::io::Result<()> {
    println!("cargo::rerun-if-changed=../AdventOfCodeInput/2015");
    for dir in read_dir("../AdventOfCodeInput/2015")
        .expect("Could not load input files from ../AdventOfCodeInput/2015")
    {
        if let Ok(dir) = dir {
            let source_path = dir.path();
            let target_path = PathBuf::from(format!(
                "./src/{}/input.txt",
                source_path
                    .with_extension("")
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
            ));
            println!("{:?} -> {:?}", source_path, target_path);
            copy(source_path, target_path)?;
        }
    }

    Ok(())
}
