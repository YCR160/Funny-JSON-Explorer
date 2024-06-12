use std::collections::HashMap;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "funny_json_explorer", about = "A funny JSON explorer.")]
struct Opt {
    /// JSON file
    #[structopt(short = "f", long = "file")]
    file: String,

    /// Style
    #[structopt(short = "s", long = "style", default_value = "tree")]
    style: String,

    /// Icon family
    #[structopt(short = "i", long = "icon", default_value = "pokerface")]
    icon: String,
}

// 创建一个图标类，提供从图标到两个字符的映射
struct Icon {
    // 从图标到两个字符的映射
    _map: HashMap<String, (char, char)>,
    non_leaf_icon: char,
    leaf_icon: char,
}

impl Icon {
    fn new(icon: &str) -> Self {
        let mut map = HashMap::new();
        map.insert("pokerface".to_string(), ('♢', '♤'));
        map.insert("heart".to_string(), ('♥', '♦'));
        let non_leaf_icon = map.get(icon).unwrap().0;
        let leaf_icon = map.get(icon).unwrap().1;
        Self {
            _map: map,
            non_leaf_icon,
            leaf_icon,
        }
    }

    fn _get(&self, name: &str) -> Option<&(char, char)> {
        self._map.get(name)
    }
}

trait Printer {
    fn build(&mut self, value: &serde_json::Value);
    fn insert(&mut self, value: &serde_json::Value);
    fn print(&mut self, icon: &Icon);
}

struct TreePrinter {
    matrix: Vec<Vec<char>>,
    child_count: usize,
    max_len: usize,
    index: usize,
    depth: usize,
}
struct RectanglePrinter {
    matrix: Vec<Vec<char>>,
    child_count: usize,
    max_len: usize,
    index: usize,
    depth: usize,
}

impl Printer for TreePrinter {
    fn build(&mut self, value: &serde_json::Value) {
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(value);
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            match node {
                serde_json::Value::Object(map) => {
                    for value in map.values() {
                        queue.push_back(value);
                    }
                }
                serde_json::Value::Array(array) => {
                    for value in array {
                        queue.push_back(value);
                    }
                }
                _ => {
                    self.child_count -= 1;
                }
            }
            self.child_count += 1;
            self.max_len = self.max_len.max(node.to_string().len());
        }
        self.matrix = vec![vec![' '; self.max_len]; self.child_count];
    }

    fn insert(&mut self, value: &serde_json::Value) {
        match value {
            serde_json::Value::Object(map) => {
                for (key, value) in map {
                    let s = key.to_string();
                    for (j, c) in s.chars().enumerate() {
                        self.matrix[self.index][j + self.depth] = c;
                    }
                    self.index += 1;
                    self.depth += 3;
                    self.insert(value);
                    self.depth -= 3;
                }
            }
            serde_json::Value::Array(array) => {
                for value in array {
                    self.insert(value);
                }
            }
            _ => {
                let s = value.to_string();
                let s = s[1..s.len() - 1].to_string();
                self.index -= 1;
                let mut j = self.max_len - 1;
                while j > 0 && self.matrix[self.index][j] == ' ' {
                    j -= 1;
                }
                self.matrix[self.index][j] = ':';
                for (k, c) in s.chars().enumerate() {
                    self.matrix[self.index][j + k + 2] = c;
                }
            }
        }
    }

    fn print(&mut self, icon: &Icon) {
        let mut first_char = vec![0; self.child_count];
        let mut max_first_char = 0;
        for i in 0..self.child_count {
            for j in 0..self.max_len {
                if self.matrix[i][j] != ' ' {
                    first_char[i] = j;
                    max_first_char = max_first_char.max(j);
                    break;
                }
            }
        }
        for i in 0..self.child_count {
            // 如果下一行的第一个非空格字符位置大于当前行的第一个非空格字符位置，说明当前行是子节点
            if i + 1 < self.child_count && first_char[i + 1] > first_char[i] {
                self.matrix[i][first_char[i] - 1] = icon.non_leaf_icon;
            } else {
                self.matrix[i][first_char[i] - 1] = icon.leaf_icon;
            }
        }
        for i in (0..self.child_count).rev() {
            if self.matrix[i][first_char[i] - 3] == ' ' {
                self.matrix[i][first_char[i] - 3] = '└';
                self.matrix[i][first_char[i] - 2] = '─';
                for j in (0..i).rev() {
                    if self.matrix[j][first_char[i] - 3] != ' ' {
                        break;
                    }
                    if self.matrix[j + 1][first_char[i] - 3] == '└'
                        || self.matrix[j + 1][first_char[i] - 3] == '├'
                        || self.matrix[j + 1][first_char[i] - 3] == '│'
                    {
                        self.matrix[j][first_char[i] - 3] = '│';
                    }
                }
            } else {
                self.matrix[i][first_char[i] - 3] = '├';
                self.matrix[i][first_char[i] - 2] = '─';
            }
        }
        for i in 0..self.child_count {
            for j in 0..self.max_len {
                print!("{}", self.matrix[i][j]);
            }
            println!();
        }
    }
}

impl Printer for RectanglePrinter {
    fn build(&mut self, value: &serde_json::Value) {
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(value);
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            match node {
                serde_json::Value::Object(map) => {
                    for value in map.values() {
                        queue.push_back(value);
                    }
                }
                serde_json::Value::Array(array) => {
                    for value in array {
                        queue.push_back(value);
                    }
                }
                _ => {
                    self.child_count -= 1;
                }
            }
            self.child_count += 1;
            self.max_len = self.max_len.max(node.to_string().len());
        }
        self.matrix = vec![vec![' '; self.max_len]; self.child_count];
    }
    fn insert(&mut self, value: &serde_json::Value) {
        match value {
            serde_json::Value::Object(map) => {
                for (key, value) in map {
                    let s = key.to_string();
                    for (j, c) in s.chars().enumerate() {
                        self.matrix[self.index][j + self.depth] = c;
                    }
                    self.index += 1;
                    self.depth += 3;
                    self.insert(value);
                    self.depth -= 3;
                }
            }
            serde_json::Value::Array(array) => {
                for value in array {
                    self.insert(value);
                }
            }
            _ => {
                let s = value.to_string();
                let s = s[1..s.len() - 1].to_string();
                self.index -= 1;
                let mut j = self.max_len - 1;
                while j > 0 && self.matrix[self.index][j] == ' ' {
                    j -= 1;
                }
                self.matrix[self.index][j] = ':';
                for (k, c) in s.chars().enumerate() {
                    self.matrix[self.index][j + k + 2] = c;
                }
            }
        }
    }
    fn print(&mut self, icon: &Icon) {
        let mut first_char = vec![0; self.child_count];
        let mut max_first_char = 0;
        for i in 0..self.child_count {
            for j in 0..self.max_len {
                if self.matrix[i][j] != ' ' {
                    first_char[i] = j;
                    max_first_char = max_first_char.max(j);
                    break;
                }
            }
        }
        for i in 0..self.child_count {
            // 如果下一行的第一个非空格字符位置大于当前行的第一个非空格字符位置，说明当前行是子节点
            if i + 1 < self.child_count && first_char[i + 1] > first_char[i] {
                self.matrix[i][first_char[i] - 1] = icon.non_leaf_icon;
            } else {
                self.matrix[i][first_char[i] - 1] = icon.leaf_icon;
            }
        }
        for i in (0..self.child_count).rev() {
            if self.matrix[i][first_char[i] - 3] == ' ' {
                self.matrix[i][first_char[i] - 3] = '├';
                self.matrix[i][first_char[i] - 2] = '─';
                for j in (0..i).rev() {
                    if self.matrix[j][first_char[i] - 3] != ' ' {
                        break;
                    }
                    if self.matrix[j + 1][first_char[i] - 3] == '├'
                        || self.matrix[j + 1][first_char[i] - 3] == '├'
                        || self.matrix[j + 1][first_char[i] - 3] == '│'
                    {
                        self.matrix[j][first_char[i] - 3] = '│';
                    }
                }
                for j in i + 1..self.child_count {
                    if self.matrix[j][first_char[i] - 3] != ' ' {
                        break;
                    }
                    if self.matrix[j - 1][first_char[i] - 3] == '├'
                        || self.matrix[j - 1][first_char[i] - 3] == '├'
                        || self.matrix[j - 1][first_char[i] - 3] == '│'
                    {
                        self.matrix[j][first_char[i] - 3] = '│';
                    }
                }
            } else {
                self.matrix[i][first_char[i] - 3] = '├';
                self.matrix[i][first_char[i] - 2] = '─';
            }
            // 将本行的最后一个非空格字符位置之后的字符设置为─
            for j in (first_char[i] + 1..self.max_len).rev() {
                if self.matrix[i][j] != ' ' {
                    self.matrix[i][j + 1] = ' ';
                    break;
                }
                self.matrix[i][j] = '─';
            }
        }
        self.matrix[0][0] = '┌';
        self.matrix[self.child_count - 1][0] = '└';
        self.matrix[0][self.max_len - 1] = '┐';
        self.matrix[self.child_count - 1][self.max_len - 1] = '┘';
        for i in 0..self.child_count {
            if self.matrix[i][self.max_len - 1] == '─' {
                self.matrix[i][self.max_len - 1] = '┤';
            }
        }
        for j in 1..first_char[self.child_count - 2] {
            if self.matrix[self.child_count - 1][j] == ' ' {
                self.matrix[self.child_count - 1][j] = '─';
            } else if self.matrix[self.child_count - 1][j] == '│'
                || self.matrix[self.child_count - 1][j] == '├'
            {
                self.matrix[self.child_count - 1][j] = '┴';
            }
        }
        for i in 0..self.child_count {
            for j in 0..self.max_len {
                print!("{}", self.matrix[i][j]);
            }
            println!();
        }
    }
}

struct PrinterFactory {
    printers: HashMap<String, Box<dyn Printer>>,
}

impl PrinterFactory {
    fn new() -> Self {
        let mut printers: HashMap<String, Box<dyn Printer>> = HashMap::new();
        printers.insert(
            "tree".to_string(),
            Box::new(TreePrinter {
                matrix: Vec::new(),
                child_count: 0,
                max_len: 0,
                index: 0,
                depth: 3,
            }),
        );
        printers.insert(
            "rectangle".to_string(),
            Box::new(RectanglePrinter {
                matrix: Vec::new(),
                child_count: 0,
                max_len: 0,
                index: 0,
                depth: 3,
            }),
        );
        Self { printers }
    }

    fn get_printer(&mut self, style: &str) -> Option<&mut Box<dyn Printer>> {
        self.printers.get_mut(style)
    }
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let file = std::fs::File::open(&opt.file).unwrap();
    let reader = std::io::BufReader::new(file);

    let value: serde_json::Value = serde_json::from_reader(reader).unwrap();

    let icon = Icon::new(&opt.icon);
    let mut factory = PrinterFactory::new();
    if let Some(printer) = factory.get_printer(&opt.style) {
        printer.build(&value);
        printer.insert(&value);
        printer.print(&icon);
    } else {
        println!("Unsupported style: {}", opt.style);
    }
}
