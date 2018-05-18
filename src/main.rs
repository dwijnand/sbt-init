//! Create a new sbt build in an existing directory
//! Author: Dale Wijnand <dale.wijnand@gmail.com>

#[allow(unused_imports)]
use std::io::{self, prelude::*};
use std::fs;

fn main() -> io::Result<()> {
    fs::create_dir("project")?;
    Ok(())
    // sbt-set-version 1.1.5
    // cat > build.sbt << EOL
    // val t = project in file(".")
    //
    // organization in ThisBuild := "com.dwijnand"
    //      version in ThisBuild := "0.1.0-SNAPSHOT"
    // scalaVersion in ThisBuild := "2.12.4"
    // EOL
    // touch project/plugins.sbt
}
