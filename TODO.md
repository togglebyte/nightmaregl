Vertex data consists of one or more 
vertex attribute pointers.

What do we need for these here
vertex pointers:

* VAO (bound) for all vertex data
* VBO, one per vertex data
    * Bound when loading data
    * Bound when adding attributes
* Divisor
    * A divisor is per attribute, 
      per index (location),
* Quad: Only load this data once 
  (and that's once, not once per render)
* Uniforms are passed directly to the shader program
  This means uniforms don't need to be involved in what we
  currently call `VertexData`
