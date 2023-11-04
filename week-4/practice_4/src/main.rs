fn main(){
   
   let fullname = "Abolusoro-Adebayo James";
   let department = "Software Engineering";
   let uni = "Pan-Atlantic University";


   let mut school = "School of Science".to_string();
   // push string
   school.push_str("and Technology");

   println!("Myname is: {}", fullname);
   // check length
   println!("The length my fullname is: {}",fullname.len());
   println!("I am a student of {} Department", department);
   println!("{}",school);
   println!("{}",uni);
}
