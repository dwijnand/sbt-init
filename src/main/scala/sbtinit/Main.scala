package sbtinit

import better.files._

object Main {
  val sbtVersion = "1.3.8"

  def main(args: Array[String]): Unit = {
    file"project".createDirectory()
    file"project/build.properties".createFile().write(s"sbt.version=$sbtVersion\n")
    file"project/plugins.sbt".createFile()
    file"build.sbt".createFile().write(s"""
      |val t = project in file(".")
      |
      |organization in ThisBuild := "com.dwijnand"
      |     version in ThisBuild := "0.1.0-SNAPSHOT"
      |scalaVersion in ThisBuild := "2.13.1"
      |""".stripMargin.stripLeading())
    file"src/main/scala/t".createDirectories()
    file"src/main/scala/t/Main.scala".createFile().write(s"""
      |package t
      |
      |object Main {
      |  def main(args: Array[String]): Unit = {
      |    println("hi")
      |  }
      |}
      |""".stripMargin.stripLeading())
  }
}
