# special_relativity_ray_caster
Edit special_realtivity_ray_tracer.ini to add models. 
To reset it to default, delete the file .

Please use only low-polygon OBJ files in this program, as it does not implement bounding volume hierarchy. Consequently, the rendering time scales linearly with the number of polygons.

This program does not need to be compiled to be used. The executable file is located in the examples folder. Additionally, the examples folder contains "default.mtl" and "default.png," both of which are necessary for the program to function properly.

These are examples of terrell rotation 
![alt text](https://github.com/Prashant-Shekhar-Rao/special_relativity_ray_caster/blob/master/examples/special_realtivity_output%20%201688982444.png)
![alt text](https://github.com/Prashant-Shekhar-Rao/special_relativity_ray_caster/blob/master/examples/special_realtivity_output%20%201688987724.png)
# # terrell rotation
The orignal terrell rotation paper assumes a small solid angle and rays striking at screen perpendicularly. The default ini file does not have a small solid angle because trying to find the object becomes tedious.
This is a modified ini file which makes sure solid angle is small
<sub>
file_location=13902_Earth_v1_l3.obj

position_of_eye= 0.0 , 0.0 , 24

height_of_camera's_screen= 4

Resolution= 1000x1000

size= 100

direction_where_eye_is_pointing= 0.0 , 0.0 ,-1.0

distance_between_focus_and_eye= 20.0

position_of_the_model=-8, 0.0 , 2.0

time=0.0

velocity_direction= -1.0 , 0, 0

velocity_magnitude_as_percentage=40

speed of light=1</sub>
In this case sphere appears spherical despite moving at 40% speed of light
![alt text](https://github.com/Prashant-Shekhar-Rao/special_relativity_ray_caster/blob/master/examples/special_realtivity_output%20%201688994496.png)

In default case with a large solid angle the sphere would appear elipsoid as illustrated here
