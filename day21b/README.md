## Day 21 Part 2 Observations

- Once a region of the board is fully "saturated", it will alternate between two states on every iteration. We don't need to constantly re-compute that region.
- If we shift the world so that the starting cell is at (0, 0), then at each step we alternate the parity of the sum of the coordinate dimensions. On even steps x + y will be even, and at odd steps it will be odd. We can hence keep track of just the cells we've visited overall, and count the ones with the proper parity at the end.