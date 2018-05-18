//! Create a new sbt build in an existing directory
//! Author: Dale Wijnand <dale.wijnand@gmail.com>

#[allow(unused_imports)]
use std::io::{self, prelude::*};
use std::path::Path;
use std::fs::{self, OpenOptions};

fn main() -> io::Result<()> {
    let project = Path::new("project");
    if !project.exists() {
        fs::create_dir(project)?;
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(project.join("build.properties"))?;
    writeln!(file, "sbt.version={}", "1.1.5")?;

    Ok(())
    // cat > build.sbt << EOL
    // val t = project in file(".")
    //
    // organization in ThisBuild := "com.dwijnand"
    //      version in ThisBuild := "0.1.0-SNAPSHOT"
    // scalaVersion in ThisBuild := "2.12.4"
    // EOL
    // touch project/plugins.sbt
}
