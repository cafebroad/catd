use std::process::Command;
use std::env;

#[test]
fn test_file_not_found() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("nonexistent_file.txt")
        .current_dir("tests") // カレントディレクトリを修正
        .output()
        .expect("failed to execute process");

    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("STDERR: {}", stderr);

    assert!(stderr.contains("Error"));
}


#[test]
fn test_no_double_space() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("people.txt")
        .current_dir("tests") // カレントディレクトリを修正
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("STDOUT: {}", stdout);

    let expected = "government of the people,\nby the people,\nfor the people\n";
    assert_eq!(stdout, expected);
}

#[test]
fn test_double_space() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("-d")
        .arg("people.txt")
        .current_dir("tests") // カレントディレクトリを修正
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("STDOUT: {}", stdout);

    let expected = "government of the people,\n\nby the people,\n\nfor the people\n\n";
    assert_eq!(stdout, expected);
}

#[test]
fn test_line_numbers_and_double_space() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("-d")
        .arg("-n")
        .arg("people.txt")
        .current_dir("tests") // カレントディレクトリを修正
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("STDOUT: {}", stdout);

    let expected = "   1: government of the people,\n\n   2: by the people,\n\n   3: for the people\n\n";
    assert_eq!(stdout, expected);
}
