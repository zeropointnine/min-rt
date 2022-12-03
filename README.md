min-rt
===

Rudimentary CPU-based raytracing library written in Rust, using the pseudocode outlined in the book ["Computer Graphics from Scratch"](https://gabrielgambetta.com/computer-graphics-from-scratch/) by Gabriel Gambetta. 

Currently working through the chapter, "Extending the Raytracer". Multithreading, camera quaternion rotation, and object transparency have been added. 

Beyond the scope of the book, the library also includes:
- ANSI TrueColor console output 
- A scene parser/generator which consumes YAML files

To run an example, `cd` to a subdirectory within `/examples`, and enter `cargo run`.

![](supporting/screenshot1.png)
