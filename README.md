# 3DTicTacToe

To computationally solve this game requires a very large search space.
I got up to searching 209800000 unique board states before my computer started hanging and I closed it. There are some optimisations that could be made:

- Implementing game with bitmaps
- Defining a standard orientation
- Tightly mapping the board to a the integers and then storing them in an array.

I don't believe that any of these optimisations will decrease the search space and increase the efficiency enough for this to be a searchable space.

_This repo should probably be called 4DTicTacToe_

## Roadmap

- [x] make scoring function absolute
- [x] fix mini max
- [x] fix almost wins with opponent centre
- [x] fix almost wins with opponent corner
- [x] refactor to interface for scoring
- [x] refactor to interface for mini max because implementing alpha beta pruning could be an extension
- [ ] refactor terminal interactions with interface + make new module
    - [ ] interface should not depend on game implementation 
    - [ ] implement local player game loop w.r to interface
    - [ ] should be asynchronously callable?
- [ ] enum based game loop
    - [ ] create game command
    - [ ] join game command
    - [ ] leave game command
    - [ ] play move command
    - [ ] there is probably a response enum
    - [ ] observer pattern is over engineered - all clients get all updates
    - [ ] predictive calculation is also overkill - just wait on server for new gamestate
- [ ] tcp server-client to forward to command pattern game loop
    - [ ] threading?
- [ ] web gui that interfaces with server client command pattern game loop 
