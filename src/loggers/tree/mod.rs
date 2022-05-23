use crate::Logger;

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
        let mut logger_tree = LoggerTree {
            loggers: root,
            children: vec![],
        };
        for log in loggers {
            let string = log.module.as_ref().expect("All Loggers Must have a module. Unless a root logger").clone();
            logger_tree.add_node_lookup(log, string);
        }
        logger_tree
    }
    pub fn find_logger(&self, path: &str) -> Option<Vec<&Logger>> {
        let mut loggers = Vec::new();
        let mut paths: Vec<&str> = path.split("::").collect();
        if paths.is_empty() {
            return None;
        }
        let current_node = paths.remove(0);
        for x in &self.children {
            if x.module.eq_ignore_ascii_case(current_node) {
                let found_loggers = x.find_logger(paths).unwrap();
                for x in found_loggers {
                    loggers.push(x);
                }
                for x in &self.loggers {
                    if x.always_execute {
                        loggers.push(x);
                    }
                }
                return Some(loggers);
            }
        }
        for x in &self.loggers {
            loggers.push(x);
        }
        Some(loggers)
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
        self.add_node_lookup(logger, path)
    }
    fn add_child(&mut self, node: TreeNode) {
        self.children.push(node);
    }
}

impl TreeNode {
    pub fn find_logger(&self, mut path: Vec<&str>) -> Option<Vec<&Logger>> {
        let mut loggers = Vec::new();

        if path.is_empty() {
            for x in &self.loggers {
                loggers.push(x);
            }
            return Some(loggers);
        }
        let current_node = path.remove(0);
        for x in &self.children {
            if x.module.eq_ignore_ascii_case(current_node) {
                let found_loggers = x.find_logger(path).unwrap();
                for x in found_loggers {
                    loggers.push(x);
                }
                for x in &self.loggers {
                    if x.always_execute {
                        loggers.push(x);
                    }
                }
                return Some(loggers);
            }
        }
        for x in &self.loggers {
            loggers.push(x);
        }
        Some(loggers)
    }
    pub fn add_node_lookup(&mut self, logger: Logger, mut path: Vec<&str>) -> bool {
        if path.is_empty() {
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
        self.add_node_lookup(logger, path)
    }
    fn add_logger(&mut self, logger: Logger) {
        self.loggers.push(logger);
    }
    fn add_child(&mut self, node: TreeNode) {
        self.children.push(node);
    }
}

