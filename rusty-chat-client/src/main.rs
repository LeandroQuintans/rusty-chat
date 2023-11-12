use std::{net::{TcpStream, SocketAddr}, io::Write};

// NOTE enums and structs are going to be shared between client and server, create its own crate for it (or place into lib.rs?)

enum Command {
    ConnectServer(SocketAddr),
    DisconnectServer,

    CreateRoom(String),
    DeleteRoom(String),
    
    JoinRoom(String),
    LeaveRoom,

    Exit,
    Help,
}

enum ErrorKind {
    CommandUnknown,
    CommandMissingArgs,
    CommandTooManyArgs,
    CommandInvalidArgs,
    
    ConnectFailed,

    RoomDoesntExist,
    RoomAlreadyExists,

    UsernameAlreadyUsed,
}

struct ChatError {
    kind: ErrorKind,
    msg: String,
}

impl ChatError {
    fn new(kind: ErrorKind, msg: String) -> ChatError {
        ChatError { kind, msg }
    }
}

impl<'a> Command {
    fn parse_connect_server(args: &Vec<&str>) -> Result<Command, ChatError> {
        if args.len() == 0 {
            Err(ChatError::new(ErrorKind::CommandMissingArgs, "Missing argument for room name".to_owned()))
        } else if args.len() > 1 {
            Err(ChatError::new(ErrorKind::CommandTooManyArgs, format!("Too many arguments. Needed: {}, Got: {}", 1, args.len())))
        } else {
            match args.last().unwrap().parse::<SocketAddr>() {
                Ok(sock_addr) => Ok(Command::ConnectServer(sock_addr)),
                Err(e) => Err(ChatError::new(ErrorKind::CommandInvalidArgs, e.to_string()))
            }
        }
    }

    fn parse_disconnect_server(args: &Vec<&str>) -> Result<Command, ChatError> {
        if args.len() != 0 {
            Err(ChatError::new(ErrorKind::CommandTooManyArgs, "Command does not need arguments".to_owned()))
        } else {
            Ok(Command::DisconnectServer)
        }
    }

    fn parse_create_room(args: &Vec<&str>) -> Result<Command, ChatError> {
        if args.len() == 0 {
            Err(ChatError::new(ErrorKind::CommandMissingArgs, "Missing argument for room name".to_owned()))
        } else if args.len() > 1 {
            Err(ChatError::new(ErrorKind::CommandTooManyArgs, format!("Too many arguments. Needed: {}, Got: {}", 1, args.len())))
        } else if !args.last().unwrap().chars().all(|c| c.is_alphanumeric()) {
            Err(ChatError::new(ErrorKind::CommandInvalidArgs, "Invalid room name, use alphanumeric characters only".to_owned()))
        } else {
            Ok(Command::JoinRoom(args.last().unwrap().to_string()))
        }
    }

    fn parse_delete_room(args: &Vec<&str>) -> Result<Command, ChatError> {
        if args.len() == 0 {
            Err(ChatError::new(ErrorKind::CommandMissingArgs, "Missing argument for room name".to_owned()))
        } else if args.len() > 1 {
            Err(ChatError::new(ErrorKind::CommandTooManyArgs, format!("Too many arguments. Needed: {}, Got: {}", 1, args.len())))
        } else if !args.last().unwrap().chars().all(|c| c.is_alphanumeric()) {
            Err(ChatError::new(ErrorKind::CommandInvalidArgs, "Invalid room name, use alphanumeric characters only".to_owned()))
        } else {
            Ok(Command::DeleteRoom(args.last().unwrap().to_string()))
        }
    }

    fn parse_join_room(args: &Vec<&str>) -> Result<Command, ChatError> {
        if args.len() == 0 {
            Err(ChatError::new(ErrorKind::CommandMissingArgs, "Missing argument for room name".to_owned()))
        } else if args.len() > 1 {
            Err(ChatError::new(ErrorKind::CommandTooManyArgs, format!("Too many arguments. Needed: {}, Got: {}", 1, args.len())))
        } else if !args.last().unwrap().chars().all(|c| c.is_alphanumeric()) {
            Err(ChatError::new(ErrorKind::CommandInvalidArgs, "Invalid room name, use alphanumeric characters only".to_owned()))
        } else {
            Ok(Command::JoinRoom(args.last().unwrap().to_string()))
        }
    }

    fn parse_leave_room(args: &Vec<&str>) -> Result<Command, ChatError> {
        if args.len() != 0 {
            Err(ChatError::new(ErrorKind::CommandTooManyArgs, "Command does not need arguments".to_owned()))
        } else {
            Ok(Command::LeaveRoom)
        }
    }

    fn parse_exit(args: &Vec<&str>) -> Result<Command, ChatError> {
        if args.len() != 0 {
            Err(ChatError::new(ErrorKind::CommandTooManyArgs, "Command does not need arguments".to_owned()))
        } else {
            Ok(Command::Exit)
        }
    }

    fn parse_help(args: &Vec<&str>) -> Result<Command, ChatError> {
        if args.len() != 0 {
            Err(ChatError::new(ErrorKind::CommandTooManyArgs, "Command does not need arguments".to_owned()))
        } else {
            Ok(Command::Help)
        }
    }

    fn parse_command(input: &str) -> Result<Command, ChatError> {
        let mut split_input: Vec<&str> = input.rsplit(" ").collect();
    
        match split_input.pop() {
            Some(str_command) => match str_command {
                "connect" => Self::parse_connect_server(&split_input),
                "disconnect" => Self::parse_disconnect_server(&split_input),

                "create" => Self::parse_create_room(&split_input),
                "delete" => Self::parse_delete_room(&split_input),

                "join" => Self::parse_join_room(&split_input),
                "leave" => Self::parse_leave_room(&split_input),

                "exit" => Self::parse_exit(&split_input),
                "help" => Self::parse_help(&split_input),

                _ => Err(ChatError::new(ErrorKind::CommandUnknown, "Command unknown or doesn't exist".to_owned())),
            },
            None => Err(ChatError::new(ErrorKind::CommandUnknown, "Command unknown or doesn't exist".to_owned())),
        }
    }


    // Run commands should be placed into a trait
    fn run_connect_server() {
        
    }

    fn run_disconnect_server() {
        
    }

    fn run_create_room() {
        
    }

    fn run_delete_room() {
        
    }

    fn run_join_room() {
        
    }

    fn run_leave_room() {
        
    }

    fn run_exit() {
        
    }

    fn run_help() {
        
    }
    
    fn run_command(&self) -> Result<(), ChatError> {
        todo!()
    }
}

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:9000").unwrap();

    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        stream.write_all(buf.as_bytes()).unwrap();
    }

    // let mut buf = String::new();
    // std::io::stdin().read_line(&mut buf).unwrap();
    // println!("{:?}", buf);
}
