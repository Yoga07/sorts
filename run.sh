#!/usr/bin/env bash
javac -cp build/classes/main/java/com/sample/test/ src/com/sample/test/*.java
cargo build
java -cp build/classes/main/java -Djava.library.path=target/debug/ com.sample.test.Main