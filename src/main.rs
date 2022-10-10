use clap::Parser;
use std::{collections::HashMap, fmt};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
    #[arg(short, long)]
    /// Action to be taken (add or list), add is used to modify by given the same task again.
    action: String,
    #[arg(short, long, default_value_t = String::new())]
    /// The job to be added to the TODO list.
    todo: String,
    #[arg(short, long, default_value_t = 0)]
    /// Is the job done or not (1 for true or  0 for false) default is false.
    is_done: u8,
}
#[derive(Debug)]
struct Todo{
    map: HashMap<String, bool>,
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Todo \t Done")?;
        let map = self.map.clone();
        for (k, v) in map {
            let box_var;
            if v{
                box_var = "[X]";
            }else{
                box_var = "[ ]";
            }
            writeln!(f, "{} \t {}", k, box_var)?;
        }
        Ok(())
    }
}

impl Todo {
    fn add_todo(&mut self, key: String, is_done: bool){
        self.map.insert(key, is_done);
    }

    fn write_todo(&mut self) -> Result<(), Box<dyn std::error::Error>>{
        
        let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open("db.json")?;
        let tmp = self.map.clone();
        match Todo::read_todo(){
            Ok(todo) =>  self.map.extend(todo.map),
            Err(err) => panic!("An error occurred: {}", err),
        }
        self.map.extend(tmp);
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    fn read_todo() -> Result<Todo, std::io::Error>{
        let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open("db.json")?;

        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e),
        }
    }
}

fn main() {
    let args = Args::parse();

    let mut todo = Todo{
        map: HashMap::new(),
    };

   match args.action.as_str(){
        "add" => {  todo.add_todo(args.todo, args.is_done == 1);
                    match todo.write_todo(){
                        Ok(()) => print!("Wrote Todo to file."),
                        Err(reason) => print!("An Error occured: {}", reason),

                    }
                },
        "list" => {print!("{}", Todo::read_todo().unwrap())},
        _ => print!("Possible actions are add or list")
    };
}

