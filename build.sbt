val sbtinit = project in file(".")
name := "sbt-init"

organization in ThisBuild := "com.dwijnand"
     version in ThisBuild := "0.1.0-SNAPSHOT"
scalaVersion in ThisBuild := "2.13.1"

enablePlugins(GraalVMNativeImagePlugin)
graalVMNativeImageOptions := List("--no-fallback")
