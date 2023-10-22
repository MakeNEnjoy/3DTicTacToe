# 3DTicTacToe

The search space for this game is very large.
I got up to searching 209800000 unique board states before my computer started hanging and I closed it. There are some optimisations that could be made:

- Implementing game with bitmaps
- Defining a standard orientation
- Tightly mapping the board to a the integers and then storing them in an array.

I don't believe that any of these optimisations will decrease the search space and increase the efficiency enough for this to be a searchable space.

_This should probably be called 4DTicTacToe_