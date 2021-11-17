pub mod console;
pub mod file;

use std::collections::HashMap;
use either::{Either, Right};
use log::{Level, logger, Record};
use log::Level::{Info, Warn};
use crate::error::Error;
use crate::loggers::console::ConsoleLogger;
use crate::loggers::TreeResponse::{NEEDS_NODE, OK};

pub struct LoggerTree {
    pub loggers: Vec<Logger>,
    pub children: Vec<TreeNode>,
}

pub struct TreeNode {
    pub module: String,
    pub loggers: Vec<Logger>,
    pub children: Vec<TreeNode>,
}


impl LoggerTree {
    pub fn new(root: Vec<Logger>, loggers: Vec<Logger>) -> LoggerTree {
        let mut logger_tree = LoggerTree { loggers: root, children: vec![] };
        for log in loggers {
            let string = log.module.clone();
            logger_tree.add_node_lookup(log, string);
        }
        return logger_tree;
    }
    pub fn find_logger(&self, path: &String) -> Option<&Vec<Logger>> {
        let mut paths: Vec<&str> = path.split("::").collect();
        if paths.len() == 0 {
            return None;
        }
        let current_node = paths.remove(0);
        for x in &self.children {
            if x.module.eq(current_node) {
                return x.find_logger(paths);
            }
        }
        return Some(&self.loggers);
    }
    pub fn add_node_lookup(&mut self, logger: Logger, path: String) {
        let mut module_path: Vec<&str> = path.split("::").collect();
        let current_node = module_path.get(0).unwrap();
        for x in &mut self.children {
            if x.module.eq(current_node) {
                module_path.remove(0);
                x.add_node_lookup(logger, module_path);
                return;
            }
        }
        let node = TreeNode {
            module: current_node.to_string(),
            loggers: vec![],
            children: vec![],
        };

        self.add_child(node);
        return self.add_node_lookup(logger, path);
    }
    fn add_logger(&mut self, logger: Logger) {
        self.loggers.push(logger);
    }
    fn add_child(&mut self, node: TreeNode) {
        self.children.push(node);
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum TreeResponse {
    OK,
    NEEDS_NODE,
}

impl TreeNode {
    pub fn find_logger(&self, mut path: Vec<&str>) -> Option<&Vec<Logger>> {
        if path.len()==0{
            return Some(&self.loggers);
        }
        let current_node = path.remove(0);
        for x in &self.children {
            if x.module.eq(current_node) {
                return x.find_logger(path);
            }
        }
        return Some(&self.loggers);
    }
    pub fn add_node_lookup(&mut self, logger: Logger, mut path: Vec<&str>) -> bool {
        if path.len() == 0 {
            self.add_logger(logger);
            return true;
        }
        let current_node = path.get(0).unwrap();
        for x in &mut self.children {
            if x.module.eq(current_node) {
             path.remove(0);
                return x.add_node_lookup(logger, path);
            }
        }
        let node = TreeNode {
            module: current_node.to_string(),
            loggers: vec![],
            children: vec![],
        };
        self.add_child(node);
        return self.add_node_lookup(logger, path);
    }
    fn add_logger(&mut self, logger: Logger) {
        self.loggers.push(logger);
    }
    fn add_child(&mut self, node: TreeNode) {
        self.children.push(node);
    }
}

#[test]
fn basic() {
    let mut loggers = Vec::new();
    loggers.push(Logger {
        module: "nitro::repo::maven".to_string(),
        levels: vec![Info],
        target: Box::new(ConsoleLogger::default()),
    });
    loggers.push(Logger {
        module: "nitro::repo".to_string(),
        levels: vec![Info],
        target: Box::new(ConsoleLogger::default()),
    });
    loggers.push(Logger {
        module: "nitro::repo::maven".to_string(),
        levels: vec![Warn],
        target: Box::new(ConsoleLogger::default()),
    });
    loggers.push(Logger {
        module: "nitro::system".to_string(),
        levels: vec![Info],
        target: Box::new(ConsoleLogger::default()),
    });
    let tree = LoggerTree::new(vec![Default::default()], loggers);
    let option = tree.find_logger(&"nitro::repo::maven".to_string()).unwrap();
    assert_eq!(option.len(), 2)
}

pub struct Logger {
    pub module: String,
    pub levels: Vec<Level>,
    pub target: Box<dyn LoggerTarget>,
}

impl Default for Logger {
    fn default() -> Self {
        return Logger {
            module: "".to_string(),
            levels: vec![],
            target: Box::new(ConsoleLogger::init(HashMap::new()).unwrap()),
        };
    }
}

impl Logger {
    pub fn module_matches(&self, module: &str) -> bool {
        if self.module.eq(module) {
            return true;
        }
        return false;
    }
}

pub trait LoggerTarget: Sync + Send {
    fn log(&self, logger: &Logger, record: &Record) -> Result<(), Error>;
}

pub trait FindLogger {
    fn find_logger(&self, record: &Record) -> Option<&Logger>;
}

impl FindLogger for Vec<Logger> {
    fn find_logger(&self, record: &Record) -> Option<&Logger> {
        for x in self {
            if x.module_matches(record.module_path().unwrap()) {
                if x.levels.contains(&record.level()) {
                    return Some(x);
                }
            }
        }
        return None;
    }
}