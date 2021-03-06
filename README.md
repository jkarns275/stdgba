# stdgba
A small rust standard library for the GBA. As of right now, I likely wont touch this project for a while. Hopefully someone finds it to be a useful reference to make rust libraries for the GBA, and the related projects I've made may help people create a working GBA rom using rust.

If you have any questions about using this, email me at josh@mail.rit.edu :)

This library intends to provide basic necessities needed to program Rust on the GBA.
That would include:
  
  - [x] (very simple) memory allocator
  - [x] dynamic allocated types (arrays and boxes)
  - [x] input handling
  - [x] image-to-tile macros (i.e. load images into static slices at runtime)
  - [ ] a complete API to the GBA's graphics functionality (partially done)
  - [ ] bindings to the GBA's link cable functionality
  - [ ] documentation and examples
  
Possible future goals would be:
  - better API design
  - math utilities
  - high-level graphics functionality
  
Help would be greatly appreciated, and suggestions are welcome - but currently the library is being developed in my limited freetime.
