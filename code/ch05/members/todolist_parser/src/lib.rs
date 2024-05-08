use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;

mod error;
use error::ParseErr;
use error::ReadErr;

#[allow(dead_code)]
pub struct TodoList {
  tasks: Vec<String>,
}

impl TodoList {
  pub fn get_todos<P>(path: P) -> Result<TodoList, Box<dyn Error>>
  where
  P: AsRef<Path>, {
      let read_todos: Result<String, Box<dyn Error>> = read_todos(path);
      let parsed_todos = parse_todos(&read_todos?)?;
      Ok(parsed_todos)
  }
}

fn parse_todos(todo_str: &str) -> Result<TodoList, Box<dyn Error>> {
  let mut tasks: Vec<String> = vec![];
  for i in todo_str.lines() {
      tasks.push(i.to_string());
  }
  if tasks.is_empty() {
      Err(ParseErr::Empty.into())
  } else {
      Ok(TodoList { tasks })
  }
}

fn read_todos<P>(path: P) -> Result<String, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let raw_todos = read_to_string(path)
        .map_err(|e| ReadErr {
            child_err: Box::new(e),
        })?;
    Ok(raw_todos)
}


#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};

    use super::*;

    #[test]
    fn test_parse_todos_valid() {
        let todo_str = "Task 1\nTask 2\nTask 3";
        let result = parse_todos(todo_str);
        assert!(result.is_ok());

        let todo_list = result.unwrap();
        assert_eq!(todo_list.tasks.len(), 3);
        assert_eq!(todo_list.tasks[0], "Task 1");
        assert_eq!(todo_list.tasks[1], "Task 2");
        assert_eq!(todo_list.tasks[2], "Task 3");
    }

    #[test]
    fn test_parse_todos_empty() {
        let todo_str = "";
        let result = parse_todos(todo_str);
        assert!(result.is_err());

        let err = result.err().unwrap();
        assert_eq!(err.downcast_ref::<ParseErr>(), Some(&ParseErr::Empty));
    }

    #[test]
    fn test_parse_todos_with_newline_at_end() {
        let todo_str = "Task 1\nTask 2\nTask 3\n";
        let result = parse_todos(todo_str);
        assert!(result.is_ok());

        let todo_list = result.unwrap();
        assert_eq!(todo_list.tasks.len(), 3);
        assert_eq!(todo_list.tasks[0], "Task 1");
        assert_eq!(todo_list.tasks[1], "Task 2");
        assert_eq!(todo_list.tasks[2], "Task 3");
    }

    #[test]
    fn test_read_todos_valid() {
      
        let path = "test_valid.txt";       
        let todo_str = "Task 1\nTask 2\nTask 3";

        std::fs::write(path, todo_str).unwrap();

        let result = read_todos(path);
        assert!(result.is_ok());

        let todos = result.unwrap();
        assert_eq!(todos, todo_str);

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_read_todos_invalid_file() {
        let path = "invalid_file.txt";
        let result = read_todos(path);
        assert!(result.is_err());

        let err = result.err().unwrap();
        assert!(err.downcast_ref::<ReadErr>().is_some());
    }

    #[test]
    fn test_get_todos_valid() {
        let path = "test_valid.txt";
        let todo_str = "Task 1\nTask 2\nTask 3";

        // Создаем файл и записываем в него тестовые данные
        let mut file = File::create(path).unwrap();
        file.write_all(todo_str.as_bytes()).unwrap();

        // Получаем задачи из файла
        let result = TodoList::get_todos(path);
        assert!(result.is_ok());

        let todos = result.unwrap();
        assert_eq!(todos.tasks.len(), 3);
        assert_eq!(todos.tasks[0], "Task 1");
        assert_eq!(todos.tasks[1], "Task 2");
        assert_eq!(todos.tasks[2], "Task 3");

        // Удаляем файл после теста
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_get_todos_invalid_file() {
        let path = "invalid_file.txt";
        
        // Получаем задачи из несуществующего файла
        let result = TodoList::get_todos(path);
        assert!(result.is_err());

        let err = result.err().unwrap();
        assert!(err.downcast_ref::<ReadErr>().is_some());
    }

    #[test]
    fn test_get_todos_invalid_content() {
        let path = "invalid_content.txt";
        let invalid_content = b"Invalid content with \x80 invalid characters";

        // Создаем файл с недопустимым содержимым
        let mut file = File::create(path).unwrap();
        file.write_all(invalid_content).unwrap();

        // Получаем задачи из файла с недопустимым содержимым
        let result = TodoList::get_todos(path);
        assert!(result.is_err());

        let err = result.err().unwrap();
        let error_info = err.downcast_ref::<ParseErr>();
        
        assert!(error_info.is_none());

        // Удаляем файл после теста
        std::fs::remove_file(path).unwrap();
    }

}
