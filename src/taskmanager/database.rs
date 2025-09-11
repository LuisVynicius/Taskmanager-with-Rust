use {
    crate::taskmanager::task::{task_status::TaskStatus, task_struct::Task},
    std::{
        fs::{File, OpenOptions},
        io::{BufRead, BufReader, Write},
        path::Path,
    },
};

pub struct Database {
    database: Vec<Task>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            database: Self::get_vec_from_database(),
        }
    }

    pub fn confirm_database_file() {
        if !Self::file_exists() {
            Self::create_file();
        }
    }

    pub fn show_tasks(&self) {
        for task in &self.database {
            println!("{task:?}");
        }
    }

    pub fn create_task(&mut self) {
        let input = std::io::stdin();

        let mut title = String::new();
        let mut description = String::new();
        let mut status = String::new();

        print!("Titulo: ");
        std::io::stdout().flush().unwrap();
        input.read_line(&mut title).unwrap();

        print!("Descrição: ");
        std::io::stdout().flush().unwrap();
        input.read_line(&mut description).unwrap();

        print!("Status: ");
        std::io::stdout().flush().unwrap();
        input.read_line(&mut status).unwrap();

        let status = status.trim().parse::<u8>().unwrap();

        let task = Task::new(
            title.trim().to_string(),
            description.trim().to_string(),
            TaskStatus::number_to_status(status),
            self.get_code() + 1,
        );

        self.database.push(task);
    }

    pub fn save(&mut self) {
        let mut file = Self::get_file(2);

        file.write(format!("{}\n", self.database.len()).as_bytes())
            .unwrap();

        for task in &self.database {
            file.write(
                format!(
                    "{} {} {} {}\n",
                    task.get_title(),
                    task.get_description(),
                    task.get_status(),
                    task.get_code()
                )
                .as_bytes(),
            ).unwrap();
        }
    }

    fn get_vec_from_database() -> Vec<Task> {
        let mut database = vec![];

        let file = Self::get_file(1);

        let buf_reader = BufReader::new(&file);

        for i in buf_reader.lines().skip(1) {
            if let Ok(line) = i {
                let line_split = line.split_whitespace().collect::<Vec<&str>>();

                let task = Task::new(
                    line_split[0].to_string(),
                    line_split[1].to_string(),
                    TaskStatus::number_to_status(line_split[2].trim().parse::<u8>().unwrap()),
                    line_split[3].trim().parse::<u16>().unwrap(),
                );

                database.push(task);
            }
        }

        database
    }

    fn file_exists() -> bool {
        Path::new("database.txt").exists()
    }

    fn create_file() {
        match File::create("database.txt") {
            Ok(_) => {}
            Err(_) => panic!("Error to generate the file."),
        }
    }

    fn get_code(&self) -> u16 {
        self.database.len() as u16
    }

    // 1 Read
    // 2 Write
    // 3 Append
    fn get_file(file_type: u8) -> File {
        let file_name = "database.txt";

        let file = match file_type {
            1 => OpenOptions::new().read(true).open(file_name),
            2 => OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(file_name),
            3 => OpenOptions::new().append(true).write(true).open(file_name),
            _ => panic!("The file_type must be a number between 1 and 3."),
        };

        match file {
            Ok(file) => return file,
            Err(_) => panic!("Error to generate the file"),
        }
    }
}
