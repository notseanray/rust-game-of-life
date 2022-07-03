Conway's Game of Life in Rust. The edge cells have a reduced amount of checks, this could be further optimized to improve speed.

The way the program is laid out allows it to easily be made parallel with the likes of Rayon. At the moment it uses zero dependencies.

This is not implemented with edge wrapping, as there is no official rule to how the borders of the grid should be implemented. It is made to easily be able to use a GUI framework such as nannou with it as well.
