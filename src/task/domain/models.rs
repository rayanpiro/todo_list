use crate::shared::task::task_id::TaskId;
use crate::task::domain::errors;

#[derive(PartialEq, Debug)]
pub struct TaskName(String);
pub struct TaskCompleted(bool);

pub struct Task {
    id: TaskId,
    name: TaskName,
    completed: TaskCompleted,
}

impl TaskName {
    fn validate(name: &String) -> bool {
        if name == "" {
            return false;
        }
        true
    }

    #[cfg(test)]
    fn bad() -> String {
        "".to_string()
    }

    #[cfg(test)]
    fn valid() -> String {
        "TestTaskName".to_string()
    }

}

impl TryFrom<String> for TaskName {

    type Error=errors::TaskError;
    fn try_from(name: String) -> Result<Self, Self::Error> {
        if !Self::validate(&name) {
            return Err(Self::Error::BadNameError)
        }
        Ok(
            TaskName(name)
        )
    }
}

impl From<TaskName> for String {
    fn from(name: TaskName) -> Self {
        name.0
    }
}

#[cfg(test)]
mod tests {
    use crate::task::domain::errors::TaskError;

    use super::TaskName;

    #[test]
    fn should_return_bad_name_error_if_task_name_is_bad() {
        let name = TaskName::try_from(TaskName::bad());
        assert_eq!(name, Err(TaskError::BadNameError))
    }

    #[test]
    fn should_return_task_name_if_is_not_bad() {
        let name = TaskName::try_from(TaskName::valid());
        assert_eq!(name, Ok(TaskName(TaskName::valid())))
    }
}