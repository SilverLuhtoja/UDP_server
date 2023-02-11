# UDP_server



# MAZEWARS

* MazeGenerator (Valeria)

TODO: 
* Seperate maze,map.rs to seperate library and import in

Lets put all same common usable values into common module ;
/common/constants.rs -> pub const BOX_SIZE: 15.0;
also needs common/mod.rs -> pub mod constants;

importing:  use crate::common::constants::{BOX_SIZE}




If you want to change size of the minimap box, then change both:
front - > player.rs ´pub const BOX_SIZE: f32 = 15.0;´
back -> map.rs ´pub const BOX_SIZE: f32 = 15.0;´
