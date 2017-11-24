fn main(){
    let path = Path::new("C:/Users/User/Desktop/DAITY/pintat.txt");
    let display = path.display();

    let mut file = match File::open(&path){
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,why.description()),
        Ok(_) => println!("{} contains:\n{}", display,s),

        }
        selection_sort(&mut file [u32]);
        println!("Sorted values are {:?}", file);
    }