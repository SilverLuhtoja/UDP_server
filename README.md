# UDP_server

PLANS TODAY:
1. Get Maze Working
2. Get player moving on map
3. Figure out what should happend when player first connects 
```
    Thought: 
    HostServer functionality (seperate library)
    1.Player tries to connect server if there is no positive response player will create server himself, wil become HostServer and setups all game data presets (list, values and so on).

    Client functionality (seperate library)
    2. Player tries to connect and there is positive feedback then it will be assigned as a client.
    WHAT IF: Player tries to connect server, but there is no positive feedback? The hostServer has disconnected and the server address is wrong.
        1. Is the HostServer creating to connections at once > one for server and one for client. This means, he is not HostServer anymore and game actually uses centralized server.
        2. another solution for that is maping to the client address, who has taken over the game. But connecting player doesn't have this info, this info has only currently playing users. It still needs a seperate server for mapping then ???

    NOTE: Every User/Client has the same setup, there is no seperations from one or another
```    
4. Figure out and write down what is saving/sending what.
5. Try to setup the overtaking logic, if host leaves the client2 will take over and game will not crash (probably not crashing anyway (UDP thing))


# MAZEWARS

* MazeGenerator (Valeria)

