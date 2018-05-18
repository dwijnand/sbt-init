//! Create a new sbt build in an existing directory
//! Author: Dale Wijnand <dale.wijnand@gmail.com>

#[allow(unused_imports)]
use std::io::{self, prelude::*};
use std::fs::{self, OpenOptions};

fn main() -> io::Result<()> {
    fs::create_dir_all("project")?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("project/build.properties")?;
    writeln!(file, "sbt.version={}", "1.1.5")?;

    fs::write("build.sbt", r#"
val t = project in file(".")

organization in ThisBuild := "com.dwijnand"
     version in ThisBuild := "0.1.0-SNAPSHOT"
scalaVersion in ThisBuild := "2.12.6"
"#.trim_left())?;

    OpenOptions::new().write(true).create(true).open("project/plugins.sbt")?;

    Ok(())
}
