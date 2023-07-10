use std::fs;

pub fn read_lines()->(f64,[f64;3],f64,[f64;2],[f64;3],String,[f64;3],f64,f64,[f64;3],f64,f64){
    //This code has pointless repetition. This code needs to be changed
    let mut result = "".to_string();
    let data=";Delete this file to reset to default settings. A new file would be created upon deletion.
;Add the location of file according to where special_relativity.exe if the file.obj is storedin same folder as object then just the name is enough.
file_location=face.obj

position_of_eye= 0.0 , 0.0 , 6.0

height_of_camera's_screen= 4
Resolution= 1000x1000
;size as percentage of total size of model
size= 100
direction_where_eye_is_pointing= 0.0 , 0.0 ,-1.0
distance_between_focus_and_eye= 2.0
;position is from our perspective at t=0
position_of_the_model= -1 , 0.0 , 2.0
;the time at which we would see the photo
time=2.0
;There is no need to normalize the vector
velocity_direction= -1.0 , 0, 0
;Set at what percent of speed of light the object is going at .90 means object is going at 90% speed of light.
velocity_magnitude_as_percentage=90
;leave next value at 1 for practical purposes it only controls how fast image changes when we change value of time. It's better to change t directly
speed of light=1";
let filename="special_realtivity_ray_tracer.ini";
let mut file=fs::read_to_string(filename);
match file {
Ok(_)=>(),
Err(_)=> {fs::write(filename, data).expect("Unable to write file");file=fs::read_to_string(filename);},

}

for line in file.unwrap().lines() {
    let mut storage=line.to_string();
     if storage.contains(';'){storage="".to_string();}
        result.push_str(&storage);
        result.push('\n');
       // result.retain(|c| !c.is_whitespace());
           
//this won't work with file
  


//file_location=

}
let mut stree="".to_string();
let mut location="".to_string();
let mut flag=false;
let mut flag2=true;
for i in result.chars(){
    if i=='\n'&&flag
    {
        flag=false;flag2=false;
    }
    if flag{
        location.push(i);
    }
stree.push(i);
    
   if stree.contains("file_location=")&&flag2{
     flag=true;
     //   
    //
    }
}
    location=location.trim().to_string();
    if !location.contains(".obj"){location.push_str(".obj");}
      result=result.replace(|c: char| !c.is_ascii(), "");
    result.retain(|c| c.is_alphanumeric()||c=='.'||c==','||c=='\n'||c=='-');
    result.to_string();
let mut stree="".to_string();
let mut storage="".to_string();
flag=false;
flag2=true;
let mut position_of_eye=[0f64;3];
let mut index=0usize;
for i in result.chars(){
    if (i=='\n'||i=='\r')&&flag&&flag2
    {
        flag=false;flag2=false;if index<3
        {
             position_of_eye[index]= storage.parse::<f64>().unwrap();
        }
        index=index+1usize;
        storage="".to_string();
    }
    if flag&&i!=','{
        storage.push(i);
    }
    else if i==','&&flag{
       // dbg!(&result);
        position_of_eye[index]= storage.parse::<f64>().unwrap();
        index=index+1usize;
        storage="".to_string();
    }
stree.push(i);
    
   if stree.contains("positionofeye")&&flag2{
     flag=true;
     //   
    //
    }
}
let mut stree="".to_string();
let mut storage="".to_string();
flag=false;
flag2=true;
let mut direction_where_eye_is_pointing=[0f64;3];
let mut index=0usize;
for i in result.chars(){
    if (i=='\n'||i=='\r')&&flag&&flag2
    {
        flag=false;flag2=false;if index<3
        {
             direction_where_eye_is_pointing[index]= storage.parse::<f64>().unwrap();
        }
        index=index+1usize;
        storage="".to_string();
    }
    if flag&&i!=','{
        storage.push(i);
    }
    else if i==','&&flag{
        direction_where_eye_is_pointing[index]= storage.parse::<f64>().unwrap();
        index=index+1usize;
        storage="".to_string();
    }
stree.push(i);
    
   if stree.contains("directionwhereeyeispointing")&&flag2{
     flag=true;
     //   
    //
    }
}
let mut stree="".to_string();
let mut storage="".to_string();
flag=false;
flag2=true;
let mut position_of_the_model=[0f64;3];
let mut index=0usize;
for i in result.chars(){
    if (i=='\n'||i=='\r')&&flag&&flag2
    {
    flag=false;flag2=false;if index<3
        {
             position_of_the_model[index]= storage.parse::<f64>().unwrap();
        }
        index=index+1usize;
        storage="".to_string();
    }
    if flag&&i!=','{
        storage.push(i);
    }
    else if i==','&&flag{
        position_of_the_model[index]= storage.parse::<f64>().unwrap();
        index=index+1usize;
        storage="".to_string();
    }
stree.push(i);
    
   if stree.contains("positionofthemodel")&&flag2{
     flag=true;
     //   
    //
    }
}
let mut stree="".to_string();
let mut storage="".to_string();
flag=false;
flag2=true;
let mut velocity_direction=[0f64;3];
let mut index=0usize;
for i in result.chars(){
    if (i=='\n'||i=='\r')&&flag&&flag2
    {
        flag=false;flag2=false;if index<3
        {
             velocity_direction[index]= storage.parse::<f64>().unwrap();
        }
        index=index+1usize;
        storage="".to_string();
    }
    if flag&&i!=','{
        storage.push(i);
    }
    else if i==','&&flag{
        velocity_direction[index]= storage.parse::<f64>().unwrap();
        index=index+1usize;
        storage="".to_string();
    }
stree.push(i);
    
   if stree.contains("velocitydirection")&&flag2{
     flag=true;
     //   
    //
    }
}
let distance_between_focus_and_eye=function_extract_single_number(&result,"distancebetweenfocusandeye".to_string());
let time=function_extract_single_number(&result,"time".to_string());
let velocity_magnitude_as_percentage=function_extract_single_number(&result,"velocitymagnitudeaspercentage".to_string());
let height_of_camera=function_extract_single_number(&result,"heightofcamerasscreen".to_string());
let size=function_extract_single_number(&result,"size".to_string());
let speed_of_light=function_extract_single_number(&result,"speedoflight".to_string());
let mut stree="".to_string();
let mut storage="".to_string();
flag=false;
flag2=true;
let mut Resolution=[0f64;2];
let mut index=0usize;
for i in result.chars(){
    if (i=='\n')&&flag&&flag2
    {
        flag=false;flag2=false;if index<2
        {
             Resolution[index]= storage.parse::<f64>().unwrap();
        }
        index=index+1usize;
        storage="".to_string();
    }
    if flag&&i!='x'{
        storage.push(i);
    }
    else if i=='x'&&flag{
        Resolution[index]= storage.parse::<f64>().unwrap();
        index=index+1usize;
        storage="".to_string();
    }
stree.push(i);
    
   if stree.contains("Resolution")&&flag2{
     flag=true;
     //   
    //
    }
}

return(time,position_of_eye,height_of_camera,Resolution,velocity_direction,location,position_of_the_model,velocity_magnitude_as_percentage,size/100f64,direction_where_eye_is_pointing,distance_between_focus_and_eye,speed_of_light);
}

fn function_extract_single_number(result:&String,phrase:String)->f64{
let mut stree="".to_string();
let mut storage="".to_string();
let mut flag=false;
let mut flag2=true;
let mut numfloat=0f64;
for i in result.chars(){
    if (i=='\n'||i=='\r')&&flag&&flag2
    {
        flag=false;flag2=false;
            numfloat= storage.parse::<f64>().unwrap();
     
    }
    if flag&&i!=','{
        storage.push(i);
    }
    else if i==','&&flag{
        numfloat= storage.parse::<f64>().unwrap();
       
        
    }
stree.push(i);
    
   if stree.contains(&phrase)&&flag2{
     flag=true;
     //   
    //
    }
}
    numfloat
}
