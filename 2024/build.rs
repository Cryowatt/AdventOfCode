use std::{
    fs::{copy, read_dir},
    path::PathBuf,
    time,
};

fn main() -> std::io::Result<()> {
    for dir in read_dir("../AdventOfCodeInput/2024")
        .expect("Could not load input files from ../AdventOfCodeInput/2024")
    {
        if let Ok(dir) = dir {
            let mut source_path = dir.path();
            let mut target_path = PathBuf::from(format!(
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
            println!("{:?}", time::Instant::now())

            // dir.path().set_extension("");
            // println!("{:?}", dir.path());
            // let mut input_file_path = PathBuf::from();
            // pat
            // let day = dir.file_name().
            // "./src/dayn/input.txt"
            // println!("{:?} {:?}", dir.file_name(), dir);
        }
        break;
    }
    panic!("fk");
}
