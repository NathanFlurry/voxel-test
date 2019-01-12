
## Notes
* We operate in u32 integers for peak performance
    * Operations on signed integers are much slower, so we just define a center index and operate based off that
    * Dividing a negative integer by a positive one will give confusing results
        * e.g. with a chunk size of 16 blocks, the index from -15 to 15 would be at chunk 0 if dividing naively
* X and Y are horizontal while Z is depth
    * It's easier to think of the world as top-down than side-on
* Building for WASM
    * Make sure [cargo-web](https://github.com/koute/cargo-web) is installed
    * Run `cargo web build`
