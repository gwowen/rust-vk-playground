My Rust Vulkan Playground
===========================================

Playing around with the ash crate, the Vulkan bindings for Rust. 

Probably only of interest to me. Don't be surprised if things don't work correctly or crash as it's just me trying out bits of code to see what I can do with them. Stuff might be broken by updates or change functionality.

## Build instructions
This requires the [Vulkan SDK from LunarG](https://vulkan.lunarg.com/) to be installed before it will run. Apart from that just running

`cargo build`

should see you through. Building in release mode:

`cargo build --release` 

will produce optimised versions of the executables that do run slightly faster (noticable with loading textures)

## Executables

There are six executables in this collection. Each can be run by running:

`cargo run --bin executable_name`

* basic_window - a basic winit window. Not very exciting.
* triangle - a Hello Triangle program. Everyone needs to write one.
* texture_map - a texture mapped square.
* depth - demonstrates the depth buffer
* cube - renders a cube to the screen
* spinny_cube - renders a spinning cube to the screen