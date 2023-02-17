# MAZEWARS

This is our version of the game [maze wars](https://www.youtube.com/watch?v=5V5X5SbSjns).

### Authors:
* Valeria Kharchenko (MazeGenerator & visual)
* Anna Lazarenkova (visual)
* Silver Luhtoja & Emil Varnomasing (connection & back)


### Task requirements:
Audit questions [here](https://github.com/01-edu/public/tree/master/subjects/multiplayer-fps/audit).<br>

*A mini map where the player can see his own position and the whole "game world".<br>
*The graphics of the game (walls and other players).<br>
*FPS on the screen.

If you want to change size of the minimap 1 box, then change BOX_SIZE in src/common/constants

### <strong> HOW TO RUN </strong> :
You can play this game locally with your friends. Just start a server and share your IP address.<br>

To run be_server or client_server:<br>

<strong> FIRST </strong>: go to mazewar directory  -->  cd mazewar

<strong> TO RUN SERVER </strong>:
- main_server --> cargo run --bin be_server 
    * result =>  logs to console your IP address<br><br>
- client_server  --> cargo run --bin mazewar  
    * result =>  ask in console to put: <br>
        1. IP address you want to connect with<br>
        2. Your player name
