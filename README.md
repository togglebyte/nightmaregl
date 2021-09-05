# NightmareGL 2D library


## Note

Don't use this. This is not ready yet. The API is still in flux and everything
might change at the drop of a hat!

### A note on sprite order:

A sprite with partial transparency can not be placed between two sprites
from a different batch. Z-index wise this will work, however the alpha blending
will not look right.
