use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub(crate) type FileLines = io::Lines<io::BufReader<File>>;

pub(crate) fn input_file_lines<P>(filename: P) -> io::Result<FileLines>
where
    P: AsRef<Path>,
{
    let project_root = env!("CARGO_MANIFEST_DIR");
    let project_root = Path::new(project_root);
    let samples_dir = project_root.join("website_inputs");
    let path = samples_dir.join(filename);
    read_lines(path)
}

fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::utils::io::input_file_lines;

    #[test]
    fn can_read_sample() {
        let file = input_file_lines("day_1.txt");
        assert!(file.is_ok());
        let res = file.unwrap();
        for line in res {
            assert!(line.is_ok());
        }
    }
}
