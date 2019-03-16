## TODO
* Chamfered edges
    * Use the existing edge data
* Vertex-based AO
    * Turn the AO in to a texture
        * Need one for every combination of corners
        * https://www.minecraftforum.net/forums/minecraft-java-edition/suggestions/25745-ambient-occlusion
* Vertex-based lighting
    * Somehow spread light?
* Texture pack tiles myself so there aren't the ugly black squares
    * Make sure to add pink null texture
* Add textures for different sides of blocks
* Add block config register
* Infinite world
* Fill edge of chunks
* Transparent textures
* Add bounding rects to only render visible items
* Simplify planes in to single chunks using greedy meshing (see https://medium.com/@fogleman/voxel-rendering-techniques-fa8d869457ca)
    * This complicates lighting + AO
* Skybox (http://onagat.hatenablog.com/entry/2017/03/24/235635)
* Chamfer the edges of the voxels
* Reference: http://www.opengl-tutorial.org/
* Process sides incrementally
    * Only when a block is changed, process sides for the blocks around it
    * Add a way to batch process sides for things like filling large areas of blocks or initial chunk generation
* Add fog so you can't see the edge of your view distance
* Make view distance circular since you don't need to load the corners
    * This also staggers the mesh generation
    * Line this up with the fog
