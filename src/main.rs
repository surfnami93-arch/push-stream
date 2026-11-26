use semver::Version;
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::io::Error;
use std::io::ErrorKind;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<(), semver::Error> {
    let path = dir();
    // cargo
    /*
    let r = input("Please enter the version (Press Enter to auto-fill): ");
    let manifest_path = path.join("Cargo.toml");
    let content = fs::read_to_string(&manifest_path);
    let mut after_text = String::new();
    let mut is_package = false;
    let owned = String::new();
    for line in content.unwrap().lines() {
        let compact: String = line.chars().filter(|c| !c.is_whitespace()).collect();

        let out: String = if compact == "[package]" {
            is_package = true;
            line.to_string()
        } else if compact.starts_with('[') {
            is_package = false; // 別のセクションに入ったら終了
            line.to_string()
        } else if is_package && compact.starts_with("version=") {
            if r.trim().is_empty() {
                // タグが無い場合: Cargo.toml の version を上げる
                let (_, after) = line.split_once('=').unwrap();
                let mut v = Version::parse(after.trim().trim_matches('"'))?;
                v.patch += 1;
                format!("version = \"{}\"", v)
            } else {
                // タグがある場合: r をそのまま使う
                format!("version = \"{}\"", r)
            }
        } else {
            line.to_string()
        };

        after_text.push_str(&out);
        after_text.push('\n');
    }
    std::fs::write(&manifest_path, after_text).unwrap();
    let cargo_path = search_command("cargo").expect("Error: unknown cargo command");
    println!("Building...");
    excuting_exe(
        &cargo_path,
        vec![
            "build".to_string(),
            "--release".to_string(),
            "--manifest-path".to_string(),
            manifest_path.display().to_string().clone(),
        ],
    );
    */
    // git base
    let git_path = search_command("git").unwrap();
    // if commit
    if path.join(".git").exists() {
        // GitHub Push
        let ver = input(
            "Please enter the version when committing to GitHub (Press Enter to auto-fill): ",
        );
        let version: String;
        if ver.trim().is_empty() {
            let output = Command::new(&git_path)
                .arg("tag")
                .arg("--sort=-version:refname")
                .output()
                .expect("tag取得中にエラーが発生しました");
            if output.stdout.is_empty() {
                /*
                println!(
                    "Since no version tag has been set, I'll match it to the version in Cargo.toml..."
                );
                version = owned.clone();
                */
                println!("Since no version number is specified on GitHub, it will be 0.1.0.");
                version = "0.1.0".to_string();
            } else {
                let s: &str = std::str::from_utf8(&output.stdout)
                    .expect("UTF-8 ではありません")
                    .trim();
                let s = s.replace("v", "");
                let mut v = Version::parse(s.as_str())?;
                v.patch += 1;
                version = format!("{}", v);
            }
        } else {
            version = ver;
        }
        let mut com = input("Enter a comment for your GitHub commit: ");
        if com.trim().is_empty() {
            com = format!("release {}", version);
        }
        excuting_exe(&git_path, vec!["add".to_string(), ".".to_string()]);
        excuting_exe(&git_path, vec!["commit".to_string(), "-m".to_string(), com]);
        excuting_exe(
            &git_path,
            vec![
                "tag".to_string(),
                "-a".to_string(),
                version.clone(),
                "-m".to_string(),
                version.clone(),
            ],
        );
        excuting_exe(
            &git_path,
            vec!["push".to_string(), "--follow-tags".to_string()],
        );
    } else {
        let link = input(
            "・Set up a remote repository
After setting up a repository on GitHub, please enter the link
https://github.com/
link: ",
        );
        // loop or version = owned;

        // version = owned;
        println!("Please create the .gitignore file in the folder beforehand.");
        let mut version = input(
            "Please enter the version when committing to GitHub(If the input is empty, the value will be 0.1.0): ",
        );
        if version.trim().is_empty() {
            version = "0.1.0".to_string();
        }
        excuting_exe(&git_path, vec!["init".to_string()]);
        excuting_exe(&git_path, vec!["add".to_string(), ".".to_string()]);
        excuting_exe(
            &git_path,
            vec![
                "commit".to_string(),
                "-m".to_string(),
                "Initial commit".to_string(),
            ],
        );
        excuting_exe(
            &git_path,
            vec![
                "tag".to_string(),
                "-a".to_string(),
                version.clone(),
                "-m".to_string(),
                version.clone(),
            ],
        );
        excuting_exe(
            &git_path,
            vec![
                "remote".to_string(),
                "add".to_string(),
                "origin".to_string(),
                link,
            ],
        );
        excuting_exe(
            &git_path,
            vec!["branch".to_string(), "-M".to_string(), "main".to_string()],
        );
        excuting_exe(&git_path, vec!["fetch".to_string(), "origin".to_string()]);
        excuting_exe(
            &git_path,
            vec![
                "push".to_string(),
                "-u".to_string(),
                "origin".to_string(),
                "main".to_string(),
                "--follow-tags".to_string(),
                "--force-with-lease".to_string(),
            ],
        );
    }
    Ok(())
}

fn input(value: &str) -> String {
    let mut input = String::new();

    print!("{}", value);
    io::stdout().flush().unwrap(); // ← これを追加

    io::stdin().read_line(&mut input).expect("入力エラー");

    input.trim().to_string()
}

fn dir() -> PathBuf {
    match env::current_dir() {
        Ok(path) => path,                             // 成功: PathBuf を取り出して返す
        Err(e) => panic!("current_dir error: {}", e), // 失敗: panic
    }
}
fn excuting_exe(path: &std::path::Path, arguments: Vec<String>) {
    Command::new(path)
        .args(arguments)
        .status()
        .expect("起動できませんでした");
}
fn search_command(cmd_name: &str) -> std::io::Result<std::path::PathBuf> {
    let path = std::env::var("PATH")
        .unwrap_or_default()
        .split(';')
        .find_map(|dir| {
            let candidate = std::path::Path::new(dir)
                .join(cmd_name)
                .with_extension("exe");
            candidate.exists().then_some(candidate)
        });

    match path {
        Some(p) => {
            return Ok(p);
        }
        None => println!("cargo not install"),
    }
    return Err(Error::new(
        ErrorKind::NotFound,
        "Please install Cargo.
            Since Rust Native is built on Cargo, it won't work without Cargo."
            .to_string(),
    ));
}
