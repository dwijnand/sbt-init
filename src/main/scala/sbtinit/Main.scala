package sbtinit

import java.nio.file.{ Files, Paths }

object Main {
  val sbtVersion = "1.3.7"

  def main(args: Array[String]): Unit = {
    Files.createDirectory(Paths.get("project"))
    Files.writeString(Files.createFile(Paths.get("project/build.properties")), s"sbt.version=$sbtVersion\n")
    Files.writeString(Files.createFile(Paths.get("project/plugins.sbt")), "")
    Files.writeString(Files.createFile(Paths.get("build.sbt")), s"""
      |val t = project in file(".")
      |
      |organization in ThisBuild := "com.dwijnand"
      |     version in ThisBuild := "0.1.0-SNAPSHOT"
      |scalaVersion in ThisBuild := "2.13.1"
      |""".stripMargin.stripLeading())
    Files.createDirectories(Paths.get("src/main/scala/t"))
    Files.writeString(Files.createFile(Paths.get("src/main/scala/t/Main.scala")), s"""
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
