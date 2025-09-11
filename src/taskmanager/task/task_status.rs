#[derive(Debug)]
pub enum TaskStatus {
    PENDENTE,
    INICIADA,
    CANCELADA,
    ENCERRADA
}

impl TaskStatus {
    pub fn number_to_status(number: u8) -> TaskStatus {
        match number {
            1 => TaskStatus::PENDENTE,
            2 => TaskStatus::INICIADA,
            3 => TaskStatus::CANCELADA,
            4 => TaskStatus::ENCERRADA,
            _ => panic!("The task_status must be a number between 1 and 4.")
        }
    }

    pub fn status_to_number(status: &TaskStatus) -> u8 {
        match &status {
            TaskStatus::PENDENTE => 1,
            TaskStatus::INICIADA => 2,
            TaskStatus::CANCELADA => 3,
            TaskStatus::ENCERRADA => 4,
        }
    }
}