# UDP_server

PLANS TODAY:
1. Get Maze Working
2. Get player moving on map
3. Figure out what should happend when player first connects 
    Thought: 
    HostServer functionality (seperate library)
    1.Player tries to connect server if there is no positive response player will create server himself, wil become HostServer and setups all game data presets (list, values and so on).

    Client functionality (seperate library)
    2. Player tries to connect qand there is positive feedback then it will be assigned as a client.

    NOTE: Every User/Client has the same setup, there is no seperations from one or another
4. Figure out and write down what is saving/sending what.
5. Try to setup the overtaking logic, if host leaves the client2 will take over and game will not crash (probably not crashing anyway (UDP thing))