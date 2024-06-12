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
        while let Some(node) = queue.pop_front() {
            if let serde_json::Value::Object(map) = node {
                queue.extend(map.values());
            } else if let serde_json::Value::Array(array) = node {
                queue.extend(array);
            } else {
                self.child_count -= 1;
            }
            self.child_count += 1;
            self.max_len = self.max_len.max(node.to_string().len());
        }
        self.matrix = vec![vec![' '; self.max_len]; self.child_count];
    }

    fn insert(&mut self, value: &serde_json::Value) {
        if let serde_json::Value::Object(map) = value {
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
        } else if let serde_json::Value::Array(array) = value {
            for value in array {
                self.insert(value);
            }
        } else {
            let s = value.to_string();
            let s = s[1..s.len() - 1].to_string();
            self.index -= 1;
            let j = self.matrix[self.index]
                .iter()
                .rposition(|&c| c != ' ')
                .unwrap_or(0);
            self.matrix[self.index][j] = ':';
            for (k, c) in s.chars().enumerate() {
                self.matrix[self.index][j + k + 2] = c;
            }
        }
    }

    fn print(&mut self, icon: &Icon) {
        let mut first_char = vec![0; self.child_count];
        let mut max_first_char = 0;
        for (i, row) in self.matrix.iter().enumerate() {
            if let Some(j) = row.iter().position(|&c| c != ' ') {
                first_char[i] = j;
                max_first_char = max_first_char.max(j);
            }
        }
        for (i, &first) in first_char.iter().enumerate() {
            let icon = match i + 1 < self.child_count && first_char[i + 1] > first {
                true => icon.non_leaf_icon,
                false => icon.leaf_icon,
            };
            self.matrix[i][first - 1] = icon;
        }
        for i in (0..self.child_count).rev() {
            match self.matrix[i][first_char[i] - 3] {
                ' ' => {
                    self.matrix[i][first_char[i] - 3] = '└';
                    self.matrix[i][first_char[i] - 2] = '─';
                    for j in (0..i).rev() {
                        if self.matrix[j][first_char[i] - 3] != ' ' {
                            break;
                        }
                        if ['└', '├', '│'].contains(&self.matrix[j + 1][first_char[i] - 3]) {
                            self.matrix[j][first_char[i] - 3] = '│';
                        }
                    }
                }
                _ => {
                    self.matrix[i][first_char[i] - 3] = '├';
                    self.matrix[i][first_char[i] - 2] = '─';
                }
            }
        }
        for (_, row) in self.matrix.iter().enumerate() {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

impl Printer for RectanglePrinter {
    fn build(&mut self, value: &serde_json::Value) {
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(value);
        while let Some(node) = queue.pop_front() {
            if let serde_json::Value::Object(map) = node {
                queue.extend(map.values());
            } else if let serde_json::Value::Array(array) = node {
                queue.extend(array);
            } else {
                self.child_count -= 1;
            }
            self.child_count += 1;
            self.max_len = self.max_len.max(node.to_string().len());
        }
        self.matrix = vec![vec![' '; self.max_len]; self.child_count];
    }

    fn insert(&mut self, value: &serde_json::Value) {
        if let serde_json::Value::Object(map) = value {
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
        } else if let serde_json::Value::Array(array) = value {
            for value in array {
                self.insert(value);
            }
        } else {
            let s = value.to_string();
            let s = s[1..s.len() - 1].to_string();
            self.index -= 1;
            let j = self.matrix[self.index]
                .iter()
                .rposition(|&c| c != ' ')
                .unwrap_or(0);
            self.matrix[self.index][j] = ':';
            for (k, c) in s.chars().enumerate() {
                self.matrix[self.index][j + k + 2] = c;
            }
        }
    }

    fn print(&mut self, icon: &Icon) {
        let mut first_char = vec![0; self.child_count];
        let mut max_first_char = 0;
        for (i, row) in self.matrix.iter().enumerate() {
            if let Some(j) = row.iter().position(|&c| c != ' ') {
                first_char[i] = j;
                max_first_char = max_first_char.max(j);
            }
        }
        for ((i, &first_char_i), row) in first_char.iter().enumerate().zip(&mut self.matrix) {
            if i + 1 < self.child_count && first_char[i + 1] > first_char_i {
                row[first_char_i - 1] = icon.non_leaf_icon;
            } else {
                row[first_char_i - 1] = icon.leaf_icon;
            }
        }
        for i in (0..self.child_count).rev() {
            match self.matrix[i][first_char[i] - 3] {
                ' ' => {
                    self.matrix[i][first_char[i] - 3] = '├';
                    self.matrix[i][first_char[i] - 2] = '─';
                    for j in (0..i).rev() {
                        if self.matrix[j][first_char[i] - 3] != ' ' {
                            break;
                        }
                        match self.matrix[j + 1][first_char[i] - 3] {
                            '├' | '│' => self.matrix[j][first_char[i] - 3] = '│',
                            _ => (),
                        }
                    }
                    for j in i + 1..self.child_count {
                        if self.matrix[j][first_char[i] - 3] != ' ' {
                            break;
                        }
                        match self.matrix[j - 1][first_char[i] - 3] {
                            '├' | '│' => self.matrix[j][first_char[i] - 3] = '│',
                            _ => (),
                        }
                    }
                }
                _ => {
                    self.matrix[i][first_char[i] - 3] = '├';
                    self.matrix[i][first_char[i] - 2] = '─';
                }
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
        for (_, cell) in self.matrix[self.child_count - 1][1..first_char[self.child_count - 2]]
            .iter_mut()
            .enumerate()
        {
            *cell = match *cell {
                ' ' => '─',
                '│' | '├' => '┴',
                _ => *cell,
            };
        }
        for (_, row) in self.matrix.iter().enumerate() {
            println!("{}", row.iter().collect::<String>());
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

    let file = std::fs::File::open(&opt.file).expect("Failed to open file");
    let reader = std::io::BufReader::new(file);

    let value: serde_json::Value = serde_json::from_reader(reader).expect("Failed to parse JSON");

    let icon = Icon::new(&opt.icon);
    let mut factory = PrinterFactory::new();
    match factory.get_printer(&opt.style) {
        Some(printer) => {
            printer.build(&value);
            printer.insert(&value);
            printer.print(&icon);
        }
        None => println!("Unsupported style: {}", opt.style),
    }
}
