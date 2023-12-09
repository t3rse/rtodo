use crate::todo_args::TodoArgs;

pub struct todo_command{
    pub cmd_type: command_type,
    pub contents: String,
}

pub enum command_type{
    add,
    list,
    remove
}

impl TryFrom<TodoArgs> for todo_command{
    type Error = ();

    fn try_from(value: TodoArgs) -> Result<Self, Self::Error> {
        if let Some(args) = value.add {
            Ok(todo_command {
                cmd_type: command_type::add,
                contents: args
            })
        }
        else if let Some(args) = value.list {
            Ok(todo_command {
                cmd_type: command_type::list,
                contents: args
            })
        }
        else if let Some(args) = value.remove {
            Ok(todo_command {
                cmd_type: command_type::remove,
                contents: args
            })
        }
        else {
            Err(())
        }

    }
}

pub struct todo_engine{
    pub command: Option<todo_command>,
}

impl todo_engine{
    pub fn new(args: TodoArgs) -> Self{
        if let Ok(command) = <TodoArgs as TryInto<todo_command>>::try_into(args) {
            todo_engine{
               command : Some(command),
            }
        }
        else {
            todo_engine {
                command: None
            }
        }
    }

    pub fn process_command(&self) -> &str {
        // TODO: process
        "The command results"
    }

}