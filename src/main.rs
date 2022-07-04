use std::env;
use std::io::Write;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file = std::fs::File::create("nomber.txt").expect("error"); 
    let ray = vec!["000","111","222","333","444","555","666","777","888","999"];
    let mut x = 0.0000001;
    for arg in &args {
        println!("{}",arg);

        if arg.chars().count() == 6{
            x = 0.0000001;
            while x < 1.0 {
                x = x + 0.0000001;
                let y = format!("{:.7}",x);
                let c = &y[2..];
                let f = format!("{}{}",arg,c);
                if !f.contains(ray[0])&&!f.contains(ray[1])&&!f.contains(ray[2])
                 &&!f.contains(ray[3])&&!f.contains(ray[4])&&!f.contains(ray[5])
                 &&!f.contains(ray[6])&&!f.contains(ray[7])&&!f.contains(ray[8])
                 &&!f.contains(ray[9]){
                    println!("{}", f);
                    file.write_all(f.as_bytes()).expect("error1");
                    file.write_all("\n".as_bytes()).expect("error2");
                    println!("Номер записан!");
                }
        }
       
    }
    else if arg.chars().count() == 7{
        x = 0.000001;
        while x < 1.0 {
            x = x + 0.0000001;
            let y = format!("{:.6}",x);
            let c = &y[2..];
            let f = format!("{}{}",arg,c);
            if !f.contains(ray[0])&&!f.contains(ray[1])&&!f.contains(ray[2])
             &&!f.contains(ray[3])&&!f.contains(ray[4])&&!f.contains(ray[5])
             &&!f.contains(ray[6])&&!f.contains(ray[7])&&!f.contains(ray[8])
             &&!f.contains(ray[9]){
                println!("{}", f);
                file.write_all(f.as_bytes()).expect("error1");
                file.write_all("\n".as_bytes()).expect("error2");
                println!("Номер записан!");
            }  
    }
    
    
        
    }
}
}
