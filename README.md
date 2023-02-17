# MAZEWARS

This is our version of the game [maze wars](https://www.youtube.com/watch?v=5V5X5SbSjns).

### Authors:
* Valeria Kharchenko (MazeGenerator & visual)
* Anna Lazarenkova (visual)
* Silver Luhtoja & Emil Varnomasing (connection & back)


### Task requirements:
*A mini map where the player can see his own position and the whole "game world".
*The graphics of the game (walls and other players).
*FPS on the screen.

Audit questions [here](https://github.com/01-edu/public/tree/master/subjects/multiplayer-fps/audit).

If you want to change size of the minimap 1 box, then change BOX_SIZE in src/common/constants

### HOW TO RUN
You can play this game locally with your friends. Just start a server and share your IP address.<br>

To run be_server or client_server:<br>
First: cd mazewar
- main_server --> cargo run --bin be_server (it will log your IP address)
- client_server  --> cargo run --bin mazewar
