
## Notes
* gfx-rs intro tutorial: https://falseidolfactory.com/2018/08/16/gfx-hal-part-0-drawing-a-triangle.html
* We operate in u32 integers for peak performance
    * Operations on signed integers are much slower, so we just define a center index and operate based off that
    * Dividing a negative integer by a positive one will give confusing results
        * e.g. with a chunk size of 16 blocks, the index from -15 to 15 would be at chunk 0 if dividing naively
* X and Y are horizontal while Z is depth
    * It's easier to think of the world as top-down than side-on
* Building for WASM
    * Make sure [cargo-web](https://github.com/koute/cargo-web) is installed
    * Run `cargo web build`
    * There's a bug in the current verison of the compiler due to the chunk structure being too large to insert in to a hashmap
* Will crash since `is_current` is false if macOS is still animating from one space to the actual game; need to report this error
    * Can't seem to reproduce this

## Requirements
* Cmake (for glsl-to-spirv)
