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
- [ ] game logging 
- [ ] i feel like the depth doesn't seem to matter a lot, investigate this further
    - [ ] measure the volatility of strategy w.r to depth
    - [ ] make a depth matrix winrate of player 1 depth vs player 2 depth.
- [ ] investigate the quality of first moves and what effect my sorting has