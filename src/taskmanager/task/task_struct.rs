use super::task_status::TaskStatus;

#[derive(Debug)]
pub struct Task {
    title: String,
    description: String,
    status: TaskStatus,
    code: u16
}

impl Task {
    pub fn new(title: String, description: String, status: TaskStatus, code: u16) -> Self {
        Self {
            title,
            description,
            status,
            code
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title[..]
    }

    pub fn get_description(&self) -> &str {
        &self.description[..]
    }

    pub fn get_status(&self) -> u8 {
        TaskStatus::status_to_number(&self.status)
    }

    pub fn set_status(&mut self, status: u8) {
        self.status = TaskStatus::number_to_status(status);
    }

    pub fn get_code(&self) -> u16 {
        self.code
    }
}