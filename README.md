# NightmareGL 2D library


## Note

### A note on sprite order:

A sprite with partial transparency can not be placed between two sprites
from a different batch. Z-index wise this will work, however the alpha blending
will not look right.
