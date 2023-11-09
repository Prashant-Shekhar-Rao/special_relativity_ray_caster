# Special relativity ray caster

## Usage
Edit special_realtivity_ray_tracer.ini to add models. To reset it to default, delete the file .
Please use only low-polygon OBJ files in this program, as it does not implement bounding volume hierarchy. Consequently, the rendering time scales linearly with the number of polygons.

This program does not need to be compiled to be used. The executable file is located in the examples folder and can be used directly. Additionally, the examples folder contains "default.mtl", "default.png" and "special_realtivity_ray_tracer.ini" which are necessary for the program to function properly.

When compiling use the release option as debug build will be very slow.
## Terrell rotation
These are examples of terrell rotation 

[Imgur](https://i.imgur.com/LpgxCVg.png)

[Imgur](https://imgur.com/HcjQtDe)

The original Terrell rotation paper (https://doi.org/10.1103%2FPhysRev.116.1041) assumes a small solid angle and rays striking the screen perpendicularly. The default ini file does not have a small solid angle because trying to find the position of the object such that it is visible on screen becomes tedious.
This is a modified ini file which makes sure solid angle is small.

file_location=13902_Earth_v1_l3.obj

position_of_eye= 0.0 , 0.0 , 24

height_of_camera's_screen= 4

Resolution= 1000x1000

size= 100

direction_where_eye_is_pointing= 0.0 , 0.0 ,-1.0

perpendicular_distance_between_viewing_plane_and_eye= 20.0

position_of_the_model=-8, 0.0 , 2.0

time=0.0

velocity_direction= -1.0 , 0, 0

velocity_magnitude_as_percentage=40

speed of light=1

**In this case sphere's outline appears spherical despite moving at 40% speed of light**

[Imgur](https://imgur.com/ugjUzgj)
This is a non moving sphere for comparison

[Imgur](https://i.imgur.com/jEbZEVC.png)
In the default case, where a large solid angle is considered, a sphere moving at 40% of the speed of light would appear as an ellipsoid, as illustrated here.

[Imgur](https://i.imgur.com/5uIPI6w.png)