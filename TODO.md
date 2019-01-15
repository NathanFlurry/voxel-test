## TODO
* Vertex-based AO
    * Need to shade the crevices
* Vertex-based lighting
    * Somehow spread light?
* Add textures for different sides of blocks
* Add block config register
* Infinite world
* Fill edge of chunks
* Transparent textures
* Add bounding rects to only render visible items
* Simplify planes in to single chunks (see https://medium.com/@fogleman/voxel-rendering-techniques-fa8d869457ca)
    * This complicates lighting + AO
* Skybox (http://onagat.hatenablog.com/entry/2017/03/24/235635)
* Texture pack tiles myself so there aren't the ugly black squares
* Chamfer the edges of the voxels
* Reference: http://www.opengl-tutorial.org/
* Process sides incrementally
    * Only when a block is changed, process sides for the blocks around it
    * Add a way to batch process sides for things like filling large areas of blocks or initial chunk generation
