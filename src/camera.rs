use vecmath as V;
use vecmath::vec3_add;
use vecmath::vec3_cross as cross;
use vecmath::vec3_dot as dot;
use vecmath::vec3_normalized;
use vecmath::vec3_scale as scale;
use vecmath::vec3_sub;
use crate::ini_reader as red;

#[derive(Debug)]
pub struct Eye {
     pub t: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
   
   
}

pub fn camera() -> Vec<Vec<Vec<u8>>> {
    let mut ini=red::read_lines();
    let c =1f64;
     //We do this division because directly changing c leads to loss of precision due to how floating point works.
     //This is equivalent to taking one light second as unit of distance 
    let div_by_light_year=1f64/ini.11;
     ini.1=scale(ini.1,div_by_light_year);
     ini.2=ini.2*div_by_light_year;
     ini.6=scale(ini.6,div_by_light_year);
     ini.8=ini.8*div_by_light_year;
     ini.10=ini.10*div_by_light_year;
     dbg!(&ini);
    let origin=&Eye{t:ini.0*c,
        x:ini.1[0],
        y:ini.1[1],
        z:ini.1[2],
    };
     let  u_bounds: (f64, f64)=(-ini.2,ini.2);
     let  v_bounds: (f64, f64)=(-ini.2*ini.3[1]/ini.3[0],ini.2*ini.3[1]/ini.3[0]);
    
     
    
    if (ini.7)>=99f64 ||ini.7<0f64
    {
        ini.7=99f64;
        }
    let Velocitiy=vecmath::vec3_normalized(ini.4);
    let mut vx =Velocitiy[0]*c*ini.7/100f64;
    let mut vy = Velocitiy[1]*c*ini.7/100f64;
    let mut vz = Velocitiy[2]*c*ini.7/100f64;
    if vx.is_nan()||vy.is_nan()||vz.is_nan(){
        if ini.7!=0f64{
         println!("velocities in ini files are inncorrect. Maybe it is 0 0 0");}
        vx=0f64;
        vy=0f64;
        vz=0f64;
    }
    let mut v_square = (vx * vx + vy * vy + vz * vz);
    if vx*vx+vy*vy+vz*vz<=000000000001f64*000000000001f64{v_square=0.000000000000000001f64;}
    let Lorentz_factor = (1.0f64 / (1.0f64 - (v_square )/ (c * c))).sqrt();
    let lorentz_factor_divided_by_c=(1.0f64 / ((c * c)- (v_square ))).sqrt();
    let lorentz_transformation_matrix: [[f64; 4]; 4] =  [
        [Lorentz_factor, -lorentz_factor_divided_by_c * vx , -lorentz_factor_divided_by_c * vy , -lorentz_factor_divided_by_c * vz ],
        [
            -lorentz_factor_divided_by_c * vx ,
            1.0f64 + (Lorentz_factor - 1.0f64) * vx * vx / (v_square),
            (Lorentz_factor - 1.0f64) * vx * vy / (v_square),
            (Lorentz_factor - 1.0f64) * vx * vz / (v_square),
        ],
        [
            -lorentz_factor_divided_by_c * vy ,
            (Lorentz_factor - 1.0f64) * vy * vx / (v_square),
            1.0f64 + (Lorentz_factor - 1.0f64) * vy * vy / (v_square),
            (Lorentz_factor - 1.0f64) * vy * vz / (v_square),
        ],
        [
            -lorentz_factor_divided_by_c * vz ,
            (Lorentz_factor - 1.0f64) * vz * vx / (v_square),
            (Lorentz_factor - 1.0f64) * vz * vy / (v_square),
            1.0f64 + (Lorentz_factor - 1.0f64) * vz * vz / (v_square),
        ],
    ];
    let mut vertexes: Vec<f64> = Vec::new();
    
    let location_of_file: String = ini.5;
    dbg!(&location_of_file);
    let cornell_box = tobj::load_obj(
        format!("{}",location_of_file),
        &tobj::GPU_LOAD_OPTIONS,
    );
    assert!(cornell_box.is_ok());

    let (models, materials) = cornell_box.unwrap();
    //if not MTL exists or file does not open the default mtl and file
   // let materials = materials.expect("Failed to load MTL file");
   let mut flag=true;
   let materials = match materials{
       Result::Ok(te)=>{te},
       Result::Err(te)=>{flag=false;tobj::load_mtl("default.mtl").expect("default.mtl is missing . Make sure it is in same directory as special_relativity.exe  ").0},
   };
    let mesh = &models[0].mesh;
    let position=[ini.6[0],ini.6[1],ini.6[2]];
    let size=ini.8;
    let position_of_center_of_model=lorentz_transformation_on_moving_object(position,&lorentz_transformation_matrix,vx,vy,vz,c);
    println!("t {}  x {}   y {}  z {}",position_of_center_of_model[0]/c,position_of_center_of_model[1],position_of_center_of_model[2],position_of_center_of_model[3]);
    for i in &mesh.indices {
        
        vertexes.push(mesh.positions[(i * 3) as usize] as f64*size + position_of_center_of_model[1]);
        vertexes.push(mesh.positions[(i * 3 + 1) as usize] as f64*size  +  position_of_center_of_model[2]);
        vertexes.push(mesh.positions[(i * 3 + 2) as usize] as f64 *size +  position_of_center_of_model[3]);
    }

    let mut textures: Vec<f64> = Vec::new();

    for i in &mesh.indices {
        textures.push(mesh.texcoords[(i * 2) as usize] as f64);
        textures.push(mesh.texcoords[(i * 2 + 1) as usize] as f64);
    }
    let mut normals: Vec<f64> = Vec::new();
    for i in &mesh.indices {
        normals.push(mesh.normals[(i * 3) as usize] as f64);
        normals.push(mesh.normals[(i * 3 + 1) as usize] as f64);
        normals.push(mesh.normals[(i * 3 + 2) as usize] as f64);
    }
    let  location_of_texture_first: String=
    match  &materials[0].diffuse_texture
    {
        Some(t)=>{t.clone()},
        None=>{"default.png".to_string()},
    }
    ;
    let mut folder=location_of_file.clone();;
    for i in location_of_file.chars().rev(){
        if(i!='\\'||i!='/')
        { folder.pop();}
     else { break };
    }
    let img = match image::open(format!("{}", location_of_texture_first)){
        Result::Ok(T)=>{T},
       Result::Err(T)=>{image::open(format!("{}", "default.png")).expect("default.png is missing. It should be in same folder as special_relativity.exe ")},
    }
        .into_rgb8();

    let height: u32 = img.height() as u32;
    let width: u32 = img.width() as u32;
    let mut image: Vec<Vec<[u8; 3]>> =
        vec![vec![[255u8, 255u8, 255u8]; height as usize]; width as usize];
    for y in 0..height {
        for x in 0..width {
            let r = img.get_pixel(x, y)[0];
            let b = img.get_pixel(x, y)[1];
            let g = img.get_pixel(x, y)[2];
            image[x as usize][y as usize] = [r, g, b];
        }
    }

    println!("dimensions {:?}", img.dimensions());


    let mut output =
        vec![vec![vec![255u8, 24u8, 157u8]; ini.3[1] as usize]; ini.3[0] as usize];
    let mut distance_from_pixel: f64;
    let f=ini.10;
    let direction_of_camera=V::vec3_normalized(ini.9);
    let mut perpendicular:[f64;3];
    if V::vec3_square_len(cross(direction_of_camera,[0f64,1f64,0f64]))>0.000001f64
    {perpendicular=V::vec3_normalized(cross(direction_of_camera,[0f64,1f64,0f64]));}
    else
    {
        perpendicular=[1f64,0f64,0f64];
    }
    let other_perpendicular=cross(direction_of_camera,perpendicular);
    let point_of_focus=V::vec3_add(scale(direction_of_camera,f),[origin.x,origin.y,origin.z]);
    let mut x: f64;
    let mut y: f64;
    let h = height as f64;
    let w = width as f64;
    let mut point_on_screen;
    for i in 0..ini.3[0] as usize {
        for j in 0..ini.3[1] as usize {
            point_on_screen=V::vec3_add(V::vec3_add(scale(perpendicular,(u_bounds.1 - u_bounds.0)*i as f64 / ini.3[0] as f64+ u_bounds.0),scale(other_perpendicular,(v_bounds.1 - v_bounds.0)*j as f64 / ini.3[1] as f64+ v_bounds.0)),[point_of_focus[0],point_of_focus[1],point_of_focus[2]]);
          
            distance_from_pixel = (((point_on_screen[0] - origin.x) * (point_on_screen[0] - origin.x)
                + (point_on_screen[1] - origin.y) * (point_on_screen[1] - origin.y)
                + (point_on_screen[2] - origin.z) * (point_on_screen[2] - origin.z)) as f64)
                .sqrt();
            output[i][j] = render(
                Ray { direction_t: (-1.0f64 *distance_from_pixel)*c,
                    direction_x: (point_on_screen[0] - origin.x),
                    direction_y: (point_on_screen[1] - origin.y) ,
                    direction_z: (point_on_screen[2] - origin.z) ,
              
                },
                origin,
                &vertexes,
                &textures,
                &normals,
                &image,
                w,
                h,
                &lorentz_transformation_matrix
            )
        }
    }
    return output;
} 
#[derive(Debug)]
struct Ray {direction_t: f64,
    direction_x: f64,
    direction_y: f64,
    direction_z: f64,
 
}
//This function primarly exists to check for what values floating point error come in and to check if lorentz_transformation_matrix is correct
fn lorentz_transformation_on_moving_object(old_Position:[f64;3],lorentz_transformation_matrix: &[[f64; 4]; 4],vx:f64,vy:f64,vz:f64,c:f64)->[f64;4]{
     
let Lorentz_factor=lorentz_transformation_matrix[0][0];
let x0=old_Position[0];
let y0=old_Position[1];
let z0=old_Position[2];
let  t=(x0*vx+y0*vy+z0*vz)*1f64/(1f64-(vx*vx+vy*vy+vz*vz)/(c*c))/(c*c);
let Position=[t*c,x0+vx*t,y0+vy*t,z0+vz*t];

return [vecmath::vec4_dot(lorentz_transformation_matrix[0],Position),vecmath::vec4_dot(lorentz_transformation_matrix[1],Position),vecmath::vec4_dot(lorentz_transformation_matrix[2],Position),vecmath::vec4_dot(lorentz_transformation_matrix[3],Position)];
}
fn lorentz_transformation_on_ray(ray_orignal: &Ray,eye_from_camera: &Eye,lorentz_transformation_matrix: &[[f64; 4]; 4])-> (Ray,Eye){

let ray_orignal=[ray_orignal.direction_t,ray_orignal.direction_x,ray_orignal.direction_y,ray_orignal.direction_z];
let eye_from_camera=[eye_from_camera.t,eye_from_camera.x,eye_from_camera.y,eye_from_camera.z];
let new_eye:Eye=Eye{
    t:vecmath::vec4_dot(lorentz_transformation_matrix[0],eye_from_camera),
    x:vecmath::vec4_dot(lorentz_transformation_matrix[1],eye_from_camera),
    y:vecmath::vec4_dot(lorentz_transformation_matrix[2],eye_from_camera),
    z:vecmath::vec4_dot(lorentz_transformation_matrix[3],eye_from_camera),
};
let new_ray:Ray=Ray{
    direction_t:vecmath::vec4_dot(lorentz_transformation_matrix[0],ray_orignal),
    direction_x:vecmath::vec4_dot(lorentz_transformation_matrix[1],ray_orignal),
    direction_y:vecmath::vec4_dot(lorentz_transformation_matrix[2],ray_orignal),
    direction_z:vecmath::vec4_dot(lorentz_transformation_matrix[3],ray_orignal),

};

return (new_ray,new_eye)
}
fn render(
    ray_old: Ray,
    origin_old: &Eye,
    vertexes: &Vec<f64>,
    textures: &Vec<f64>,
    normals: &Vec<f64>,
    image: &Vec<Vec<[u8; 3]>>,
    width: f64,
    height: f64,
    lorentz_transformation_matrix:&[[f64;4];4]
) -> Vec<u8> {
    
    let (ray,origin)=lorentz_transformation_on_ray(&ray_old,origin_old,lorentz_transformation_matrix);
    let point_on_ray = [origin.x, origin.y, origin.z];
    let mut smallest_distance_of_polygon_from_eye = 99999999999999999999999.0f64;
  

    let mut outside;
    let mut rett = vec![0u8, 0u8, 0u8];
  
     let ray=vec3_normalized([ray.direction_x, ray.direction_y, ray.direction_z]);
  
    for i in (0..(vertexes.len() - 8)).step_by(9usize) {
        
       
        let mut v1 = [vertexes[i], vertexes[i + 1], vertexes[i + 2]];
        let mut v2 = [vertexes[i + 3], vertexes[i + 4], vertexes[i + 5]];
        let mut v3 = [vertexes[i + 6], vertexes[i + 7], vertexes[i + 8]];
     
        let cross_product: [f64; 3];
        if normals.len() != 0 {
            cross_product = [normals[i], normals[i + 1], normals[i + 2]];
        } else {
            cross_product = V::vec3_normalized(cross(vec3_sub(v2, v1), vec3_sub(v3, v1)));
        }
       
        let vec3_dot = dot(
            ray,
            cross_product,
        );
        let not_skip: bool = vec3_dot < 0.0;

        if not_skip {
            let d = dot(
                [
                    v1[0] - point_on_ray[0],
                    v1[1] - point_on_ray[1],
                    v1[2] - point_on_ray[2],
                ],
                cross_product,
            ) / vec3_dot;
            if smallest_distance_of_polygon_from_eye > d && d > 0f64 {
                let a = i / 3 * 2;
                let mut v1_texture = [textures[a], textures[a + 1]];
                let mut v2_texture = [textures[a + 2], textures[a + 3]];
                let mut v3_texture = [textures[a + 4], textures[a + 5]];
                let dd12 = V::vec3_square_len(V::vec3_sub(v1, v2));
                let dd31 = V::vec3_square_len(V::vec3_sub(v1, v3));
                let dd23 = V::vec3_square_len(V::vec3_sub(v2, v3));
                 if dd12 >= dd23 && dd12 >= dd31 {
                } else if dd23 >= dd12 && dd23 >= dd31 {
                    let holding = v3;
                    v1_texture = [textures[a + 2], textures[a + 3]];
                    v2_texture = [textures[a + 4], textures[a + 5]];
                    v3_texture = [textures[a], textures[a + 1]];
                    v3 = v1;
                    v1 = v2;
                    v2 = holding;
                } else {
                    v1_texture = [textures[a + 4], textures[a + 5]];
                    v2_texture = [textures[a], textures[a + 1]];
                    v3_texture = [textures[a + 2], textures[a + 3]];
                    let holding = v3;
                    v3 = v2;
                    v2 = v1;
                    v1 = holding;
                }
                let a1 = vec3_sub(v2, v1);
                let a2 = vec3_sub(v3, v1);
                

                let point = vec3_add(
                    point_on_ray,
                    scale(ray, d),
                );
               
                let eu = vec3_normalized(a1);
                let ev = vec3_normalized(vec3_sub(a2, scale(eu, dot(eu, a2))));
                let w = dot(eu, a1);
                // let w=V::vec3_square_len(a1);
                let g = dot(eu, a2);
                let h = dot(ev, a2);
                let u = dot(eu, vec3_sub(point, v1));
                let v = dot(ev, vec3_sub(point, v1));
                outside = u < 0.0f64
                    || u > w
                    || v < 0.0f64
                    || v > h
                    || u <= g && v > u * h / g
                    || u >= g && v > (w - u) * h / (w - g);
              
                let barycentric_u = -u / w + g / w / h * v - v / h + 1f64;
                let barycentric_v = u / w - g / w / h * v;
              
                if outside == false && barycentric_u <= 1.0f64 && barycentric_v <= 1f64 {
                  
                    let mut location_of_point_on_texture = V::vec2_add(
                        V::vec2_scale(v1_texture, barycentric_u),
                        V::vec2_add(
                            V::vec2_scale(v2_texture, barycentric_v),
                            V::vec2_scale(v3_texture, 1.0f64 - barycentric_u - barycentric_v),
                        ),
                    );
                    location_of_point_on_texture = [
                        location_of_point_on_texture[0]
                            - location_of_point_on_texture[0].floor() as f64,
                        location_of_point_on_texture[1]
                            - location_of_point_on_texture[1].floor() as f64,
                    ];
let mut coeff=vec3_dot.abs();
if coeff <0.1 {coeff=0.1;}
                    rett = vec![
                       
                        (image[(location_of_point_on_texture[0] * (width-1f64)) as usize]
                            [((1.0f64 - location_of_point_on_texture[1]) * (width-1f64)) as usize][0]
                            as f64
                            *coeff ) as u8,
                        (image[(location_of_point_on_texture[0] * (width-1f64)) as usize]
                            [((1.0f64 - location_of_point_on_texture[1]) * (height-1f64)) as usize][1]
                            as f64
                            * coeff) as u8,
                        (image[(location_of_point_on_texture[0] * (width-1f64)) as usize]
                            [((1.0f64 - location_of_point_on_texture[1]) * (height-1f64)) as usize][2]
                            as f64
                            * coeff) as u8,
                    ]; 
                    smallest_distance_of_polygon_from_eye = d;
                }
            }
        }
    }
    return rett;
}
