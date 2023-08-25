# lumos_lib
A Rust port of my dad's [Lumos library](https://github.com/john-petrangelo/lumos-arduino), originally written in C++.

This repository also contains a client web app to play with the Lumos library without an Arduino with an LED strip.

To run the client web app, run the `deploy.sh` script, which will add the wasm build target, build to that target,
install trunk, and use that to serve the app.
