use std::fs::File;
use std::io::{BufReader, prelude::*};

fn main(){
   let file = File::open("input").unwrap();
   let reader = BufReader::new(file);
   let mut so: Vec<Scob> = Vec::new();
   let mut y: i32 =0;

   #[derive(Debug)]
   struct Scob {
       id: String,
       x:  i32,
       y:  i32,
       ls: usize,
       f: bool,
    }

   for line in reader.lines(){
       let st = line.unwrap();
       println!("{}",st);

       let mut t_st = "".to_string();
       let mut v: Vec<String> = Vec::new();

       let mut x: i32 = 0;
       for ch in st.chars() {
           if ch == '0' || ch == '1' || ch == '2' || ch == '3' || ch == '4' || ch == '5' || ch == '6' || ch == '7' || ch == '8' || ch == '9' {
               t_st.push(ch);
           } else {
               if t_st !="" {
                   let ost = Scob {id:t_st.to_string(), x:x, y:y, ls:t_st.to_string().len(), f: true};
                   so.push(ost);

                   v.push(t_st.to_string());
                   t_st = "".to_string();
               }
           }
           x = x+1;
       }
       let mut x: i32 = 0;
       for ch in st.chars() {
             if ch != '0' && ch != '1' && ch != '2' && ch != '3' && ch != '4' && ch != '5' && ch != '6' && ch != '7' && ch != '8' && ch != '9' && ch !='.' {
                t_st.push(ch);
             } else {
                if t_st !="" {
                    let ost = Scob {id:t_st.to_string(), x:x, y:y, ls:t_st.to_string().len(), f:false};
                    so.push(ost);

                    v.push(t_st.to_string());
                    t_st = "".to_string();
                 }
             }
             x =x +1;
       }
       y = y+1;
    }
//    println!("{}", so.len());
    for a in 0..so.len() {
        println!("{:?}",so[a]);
        if so[a].f {
            let b = so[a].id.parse::<i32>().unwrap();
            println!("{}", b);
            for l in a+1..so.len() {
                if so[l].f {
                   println!("{} ",so[l].id.parse::<i32>().unwrap());
                }
            }
        } else {
             println!("Символ.");
        }
     }

}
