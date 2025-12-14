use {
    crate::taskmanager::{task::{task_status::TaskStatus, task_struct::Task}, utils::read_number},
    std::{
        fs::{File, OpenOptions}, io::{BufRead, BufReader, Write}, path::Path
    },
};

pub struct Database {
    database: Vec<Task>,
    code: u16,
}

impl Database {
    pub fn new() -> Self {
        Self {
            database: Self::get_vec_from_database(),
            code: Self::get_code_from_database(),
        }
    }

    pub fn confirm_database_file() {
        if !Self::file_exists() {
            Self::create_file();
        }
    }

    pub fn show_tasks(&self) {
        println!("[Id] | [Título] | [Descrição] | [Status]");
        for task in &self.database {
            println!("{task}");
        }
    }

    pub fn create_task(&mut self) {
        let input = std::io::stdin();

        let mut title = String::new();
        let mut description = String::new();
        let mut status_string = String::new();

        print!("Titulo: ");
        std::io::stdout().flush().unwrap();
        input.read_line(&mut title).unwrap();

        print!("Descrição: ");
        std::io::stdout().flush().unwrap();
        input.read_line(&mut description).unwrap();

        print!("Status: ");
        std::io::stdout().flush().unwrap();
        input.read_line(&mut status_string).unwrap();

        let status = match status_string.trim().parse::<u8>() {
            Ok(number) => number,
            Err(_) => {
                println!("O status deve ser um número de 1 à 4, status definido para 1(PENDENTE)");
                1
            }
        };

        let task = Task::new(
            title.trim().to_string(),
            description.trim().to_string(),
            TaskStatus::number_to_status(status),
            self.code,
        );

        self.database.push(task);
        self.code += 1;
    }

    pub fn delete_by_name(&mut self) {
        let mut name = String::new();

        print!("Nome da tarefa: ");
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut name).unwrap();

        let name = name.trim();

        if self.exists_by_title(name) {
            self.database.remove(
                self.database
                    .iter()
                    .position(|task| task.get_title() == name)
                    .unwrap(),
            );
        }
    }

    pub fn delete_by_position(&mut self) {
        let position = read_number();

        let position= match position {
            Some(number) => number,
            None => {
                println!("A posição deve ser um número");
                return;
            }
        };

        self.database.remove(position as usize);
    }

    pub fn delete_by_code(&mut self) {
        let code = read_number();

        let code = match code {
            Some(number) => number as u16,
            None => {
                println!("O código deve ser um número");
                return;
            }
        };

        if self.exists_by_code(code) {
            self.database.remove(
                self.database
                    .iter()
                    .position(|task| task.get_code() == code)
                    .unwrap(),
            );
        } else {
            println!("Tarefa não encontrada por código");
        }
    }

    pub fn save(&mut self) {
        let mut file = Self::get_file(2);

        file.write(format!("{}\n", self.code).as_bytes()).unwrap();

        for task in &self.database {
            file.write(
                format!(
                    "{};{};{};{}\n",
                    task.get_title(),
                    task.get_description(),
                    task.get_status(),
                    task.get_code()
                )
                .as_bytes(),
            )
            .unwrap();
        }
    }

    pub fn find_by_name(&self) {
        let mut name = String::new();

        print!("Nome da tarefa: ");
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut name).unwrap();

        let task = self
            .database
            .iter()
            .find(|task| task.get_title() == name.trim());

        match task {
            Some(t) => println!("{t}"),
            None => println!("Não foi encontrado uma tarefa com esse nome"),
        }
    }

    pub fn find_by_position(&self) {
        let position = read_number();

        let position = match position {
            Some(number) => number,
            None => {
                println!("A posição deve ser um número");
                return;
            }

        } as usize;

        let task = self.database.get(position - 1);

        match task {
            Some(t) => println!("{t}"),
            None => println!("Não foi encontrado uma tarefa com essa Posição"),
        }
    }

    pub fn find_by_code(&self) {
        
        print!("Código da tarefa: ");
        let code = read_number();

        let code = match code {
            Some(number) => number as u16,
            None => {
                println!("O código deve ser um número");
                return;
            }
        };

        match self.database.get(self.position_by_code(code)) {
            Some(t) => println!("{t}"),
            None => println!("Não foi encontrado uma tarefa com esse Código"),
        }
    }

    pub fn change_status(&mut self) {
        print!("Código da tarefa: ");
        
        let code = read_number();

        let code = match code {
            Some(number) => number as u16,
            None => {
                println!("O código deve ser um número");
                return;
            }
        };
        
        if !self.exists_by_code(code) {
            println!("Nenhuma tarefa com esse código foi encontrada");
            return;
        }

        let position = self.position_by_code(code);

        let task = &mut self.database[position];

        println!("{task}");

        println!("\n1ºPENDENTE\n2ºINICIADA\n3ºCANCELADA\n4ºENCERRADA\n");

        print!("Digite seu novo status: ");

        std::io::stdout().flush().unwrap();

        let status = read_number();

        let status = match status {
            Some(number) if number <= 0 || number > 4 => {
                println!("O status deve ser um número entre 1-4.");
                return;
            },
            Some(number) => number,
            None => {
                println!("O status deve ser um número entre 1-4.");
                return;
            }
        };

        task.set_status(status);
        

    }

    fn position_by_code(&self, code: u16) -> usize {
        self.database.iter().position(|t| t.get_code() == code).unwrap()
    }

    fn exists_by_title(&self, title: &str) -> bool {
        self.database.iter().any(|task| task.get_title() == title)
    }

    fn exists_by_code(&self, code: u16) -> bool {
        self.database.iter().any(|task| task.get_code() == code)
    }

    fn get_vec_from_database() -> Vec<Task> {
        let mut database = vec![];

        let file = Self::get_file(1);

        let buf_reader = BufReader::new(&file);

        for i in buf_reader.lines().skip(1) {
            if let Ok(line) = i {
                let line_split = line.split(';').collect::<Vec<&str>>();

                let task = Task::new(
                    line_split[0].to_string(),
                    line_split[1].to_string(),
                    TaskStatus::number_to_status(
                        line_split[2].trim().parse::<u8>().unwrap()
                    ),
                    line_split[3].trim().parse::<u16>().unwrap(),
                );

                database.push(task);
            }
        }

        database
    }

    fn get_code_from_database() -> u16 {
        let buf_reader = BufReader::new(Self::get_file(1));

        match buf_reader.lines().nth(0) {
            Some(x) => x.unwrap().parse::<u16>().unwrap(),
            None => 1,
        }
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
            _ => panic!("The file_type must be a number between 1 and 2."),
        };

        match file {
            Ok(file) => return file,
            Err(_) => panic!("Error to generate the file"),
        }
    }
}
