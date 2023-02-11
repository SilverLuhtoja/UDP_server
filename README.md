# MAZEWARS

* MazeGenerator (Valeria)
* Visual (Anna)
* Connection (Silver & Emil)

If you want to change size of the minimap box, then change both:
- front - > common/constants
- back -> map.rs ´pub const BOX_SIZE: f32 = 15.0;´


to run be_server or client_server;
- be_server --> cargo run --bin be_server
- client_server  --> cargo run --bin mazewar