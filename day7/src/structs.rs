use fmt_derive::Debug;
use regex::Regex;
use std::{cell::RefCell, rc::Rc};
type CellNode = Rc<RefCell<Node>>;

#[derive(Debug)]
pub enum Node {
    File {
        name: String,
        size: usize,
    },
    Dir {
        name: String,
        children: Vec<CellNode>,
        #[debug(ignore)]
        parent: Option<CellNode>,
    },
}

impl Node {
    fn size(&self) -> usize {
        match self {
            Node::File { name: _, size } => *size,
            Node::Dir {
                name: _,
                children,
                parent: _,
            } => children.iter().map(|node| node.borrow().size()).sum(),
        }
    }
}

impl Node {
    fn parse(line: String) -> Self {
        let mut split = line.split(" ");
        let lead = split.next().unwrap();
        let follow = split.next().unwrap();
        if lead == "dir" {
            Node::Dir {
                name: follow.to_string(),
                children: Vec::new(),
                parent: None,
            }
        } else {
            Node::File {
                name: follow.to_string(),
                size: lead.parse().unwrap(),
            }
        }
    }
}

#[derive(Debug)]
pub struct Shell {
    root: CellNode,
    current: CellNode,
}

impl Shell {
    pub fn new() -> Self {
        let root = Rc::new(RefCell::new(Node::Dir {
            name: "/".into(),
            children: Vec::new(),
            parent: None,
        }));
        Shell {
            current: root.clone(),
            root,
        }
    }

    pub fn apply_command(&mut self, command: &Command) {
        match command {
            Command::ChangeDir { target } => match target.as_str() {
                "/" => (),
                ".." => self.move_up(),
                target => self.move_to(target.to_string()),
            },
            Command::ListDir { content } => {
                self.fill_node(content);
            }
            _ => (),
        }
    }

    fn move_to(&mut self, target: String) {
        let mut carrier = None;
        if let Node::Dir {
            parent: _,
            name: _,
            children,
        } = &*self.current.borrow()
        {
            let node = children.iter().find(|node| {
                if let Node::Dir {
                    name,
                    children: _,
                    parent: _,
                } = &*node.borrow()
                {
                    name.cmp(&target).is_eq()
                } else {
                    false
                }
            });
            let node =
                node.expect(format!("Can not move to {} from {:?}", target, self.current).as_str());
            carrier = Some(node.clone());
        }
        self.current = carrier.unwrap();
    }

    fn move_up(&mut self) {
        let mut carrier = None;
        if let Node::Dir {
            parent,
            name: _,
            children: _,
        } = &*self.current.borrow()
        {
            let parent_node = parent
                .as_ref()
                .expect(format!("Can not move to .. from {:?}", self.current).as_str());
            carrier = Some(parent_node.clone());
        }
        self.current = carrier.unwrap();
    }

    fn fill_node(&mut self, content: &Vec<CellNode>) {
        if let Node::Dir {
            name: _,
            children,
            parent: _,
        } = &mut *self.current.borrow_mut()
        {
            content.iter().for_each(|node| {
                let node = node.clone();
                if let Node::Dir {
                    name: _,
                    children: _,
                    parent,
                } = &mut *node.borrow_mut()
                {
                    *parent = Some(self.current.clone());
                }
                children.push(node);
            })
        }
    }

    pub fn search(&self, size_at_most: usize) -> Vec<SearchResult> {
        let mut result = Vec::new();
        Self::search_internal(&mut result, self.root.clone(), size_at_most);
        result
    }

    fn search_internal(result: &mut Vec<SearchResult>, node: CellNode, size_at_most: usize) {
        if let Node::Dir {
            parent: _,
            name: _,
            children,
        } = &*node.borrow()
        {
            let size = node.borrow().size();
            if size <= size_at_most {
                result.push(SearchResult {
                    node: node.clone(),
                    size,
                })
            }
            children
                .iter()
                .for_each(|node| Self::search_internal(result, node.clone(), size_at_most))
        }
    }
}

pub struct SearchResult {
    pub node: CellNode,
    pub size: usize,
}

#[derive(Debug)]
pub enum Command {
    ChangeDir { target: String },
    ListDir { content: Vec<CellNode> },
    Output(String),
}

impl Command {
    fn new_list_dir() -> Self {
        Self::ListDir {
            content: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct FlatCommands {
    commands: Vec<Command>,
}

impl FlatCommands {
    pub fn parse(content: String) -> Self {
        let mut commands = Vec::new();
        let command_pattern = Regex::new(r"^\$ (\w+)\s?(\w+|/|\.\.)?").unwrap();
        for line in content.lines() {
            if let Some(captures) = command_pattern.captures(line) {
                let command = captures.get(1).unwrap().as_str();
                if command == "cd" {
                    let target = captures.get(2).unwrap().as_str().to_string();
                    commands.push(Command::ChangeDir { target });
                } else if command == "ls" {
                    commands.push(Command::new_list_dir());
                }
            } else {
                commands.push(Command::Output(line.to_string()));
            }
        }
        Self { commands }
    }

    pub fn compact(mut self) -> RecordedCommands {
        let mut commands = Vec::new();
        self.commands.reverse();
        loop {
            let command = self.commands.pop();
            if command.is_none() {
                break;
            }
            let command = command.unwrap();
            match command {
                Command::ChangeDir { target: _ } | Command::ListDir { content: _ } => {
                    commands.push(command)
                }
                Command::Output(line) => {
                    if let Some(Command::ListDir { content }) = commands.last_mut() {
                        let node = Node::parse(line.clone());
                        content.push(Rc::new(RefCell::new(node)));
                    }
                }
            }
        }
        RecordedCommands { commands }
    }
}

#[derive(Debug)]
pub struct RecordedCommands {
    commands: Vec<Command>,
}

impl IntoIterator for RecordedCommands {
    type Item = Command;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.commands.into_iter()
    }
}
