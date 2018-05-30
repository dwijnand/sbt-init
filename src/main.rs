//! Create a new sbt build in an existing directory
//! Author: Dale Wijnand <dale.wijnand@gmail.com>

use std::fs;
use std::io;

fn main() -> io::Result<()> {
    fs::create_dir_all("project")?;
    fs::write("project/build.properties", "sbt.version=1.1.5")?;
    fs::write("project/plugins.sbt", "")?;
    fs::write("build.sbt", r#"
val t = project in file(".")

organization in ThisBuild := "com.dwijnand"
     version in ThisBuild := "0.1.0-SNAPSHOT"
scalaVersion in ThisBuild := "2.12.6"
"#.trim_left())?;
    Ok(())
}
