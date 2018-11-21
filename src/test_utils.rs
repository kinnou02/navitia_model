// Copyright 2017-2018 Kisio Digital and/or its affiliates.
//
// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see
// <http://www.gnu.org/licenses/>.

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
extern crate tempdir;
use std::path;

use self::tempdir::TempDir;

pub fn compare_output_dir_with_expected(
    output_dir: &Path,
    files_to_check: &[String],
    work_dir_expected: String,
) {
    for filename in files_to_check {
        let output_file_path = output_dir.join(filename);
        let mut output_file = File::open(output_file_path.clone())
            .expect(&format!("file {:?} not found", output_file_path));
        let mut output_contents = String::new();
        output_file.read_to_string(&mut output_contents).unwrap();
        let expected_file_path = format!("{}/{}", work_dir_expected, filename);
        let mut expected_file = File::open(expected_file_path.clone())
            .expect(&format!("file {} not found", expected_file_path));
        let mut expected_contents = String::new();
        expected_file
            .read_to_string(&mut expected_contents)
            .unwrap();
        assert_eq!(output_contents, expected_contents);
    }
}

pub fn create_file_with_content(path: &path::Path, file_name: &str, content: &str) {
    let file_path = path.join(file_name);
    let mut f = File::create(&file_path).unwrap();
    f.write_all(content.as_bytes()).unwrap();
}

pub fn test_in_tmp_dir<F>(func: F)
where
    F: FnOnce(&path::Path),
{
    let tmp_dir = TempDir::new("navitia_model_tests").expect("create temp dir");
    {
        let path = tmp_dir.as_ref();
        func(path);
    }
    tmp_dir.close().expect("delete temp dir");
}
