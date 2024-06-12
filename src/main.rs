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
    non_leaf_icon: String,
    leaf_icon: String,
}

impl Icon {
    fn new(icon: &str) -> Self {
        let mut map = HashMap::new();
        map.insert("pokerface".to_string(), ('♤', '♢'));
        map.insert("heart".to_string(), ('♥', '♦'));
        let non_leaf_icon = map.get(icon).unwrap().0.to_string();
        let leaf_icon = map.get(icon).unwrap().1.to_string();
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
    fn print(&self, value: &serde_json::Value, prefix: String, icon: &Icon, depth: usize);
}

struct TreePrinter;
struct RectanglePrinter;

impl Printer for TreePrinter {
    fn print(&self, value: &serde_json::Value, prefix: String, icon: &Icon, depth: usize) {
        match value {
            serde_json::Value::Object(map) => {
                let mut child_count = 0;
                for value in map.values() {
                    match value {
                        serde_json::Value::Object(inner_map) => {
                            child_count += inner_map.len();
                        }
                        serde_json::Value::Array(inner_array) => {
                            child_count += inner_array.len();
                        }
                        _ => {}
                    }
                }
                if child_count == 0 {
                    let len = map.len();
                    let mut i = 0;
                    for (key, value) in map {
                        let mut my_prefix = " ".repeat(depth * 2);
                        for (i, c) in prefix.chars().enumerate() {
                            if i < my_prefix.len() {
                                my_prefix.replace_range(i..i + 1, &c.to_string());
                            }
                        }
                        my_prefix.replace_range(
                            my_prefix.len() - 2..my_prefix.len(),
                            if i == len - 1 { "L_" } else { "|-" },
                        );
                        print!("\n{}{}{}", my_prefix, icon.leaf_icon, key);
                        self.print(value, my_prefix, icon, depth + 1);
                        i += 1;
                    }
                } else {
                    let len = map.len();
                    let mut i = 0;
                    for (key, value) in map {
                        let mut my_prefix = " ".repeat(depth * 2);
                        for (i, c) in prefix.chars().enumerate() {
                            if i < my_prefix.len() {
                                my_prefix.replace_range(i..i + 1, &c.to_string());
                            }
                        }
                        my_prefix.replace_range(
                            my_prefix.len() - 2..my_prefix.len(),
                            if i == len - 1 { "L_" } else { "|-" },
                        );
                        print!("\n{}{}{}", my_prefix, icon.non_leaf_icon, key);
                        my_prefix = " ".repeat(depth * 2);
                        my_prefix.replace_range(
                            my_prefix.len() - 2..my_prefix.len(),
                            if i == len - 1 { "  " } else { "| " },
                        );
                        self.print(value, my_prefix, icon, depth + 1);
                        i += 1;
                    }
                }
            }
            _ => {
                print!(": {}", value);
            }
        }
    }
}

impl Printer for RectanglePrinter {
    fn print(&self, value: &serde_json::Value, prefix: String, icon: &Icon, depth: usize) {
        match value {
            serde_json::Value::Object(map) => {
                let mut child_count = 0;
                for value in map.values() {
                    match value {
                        serde_json::Value::Object(inner_map) => {
                            child_count += inner_map.len();
                        }
                        serde_json::Value::Array(inner_array) => {
                            child_count += inner_array.len();
                        }
                        _ => {}
                    }
                }
                if child_count == 0 {
                    for (key, value) in map {
                        let mut my_prefix = "-".repeat(43);
                        my_prefix.replace_range(42..43, "|");
                        for (i, c) in prefix.chars().enumerate() {
                            if i < my_prefix.len() {
                                my_prefix.replace_range(i..i + 1, &c.to_string());
                            }
                        }
                        my_prefix.replace_range(depth * 3 - 3..depth * 3, "|- ");
                        my_prefix.replace_range(depth * 3..depth * 3 + key.len(), key);
                        print!("\n{}", my_prefix);
                        self.print(value, my_prefix, icon, depth + 1);
                    }
                } else {
                    for (key, value) in map {
                        let mut my_prefix = "-".repeat(43);
                        my_prefix.replace_range(42..43, "|");
                        for (i, c) in prefix.chars().enumerate() {
                            if i < my_prefix.len() {
                                my_prefix.replace_range(i..i + 1, &c.to_string());
                            }
                        }
                        my_prefix.replace_range(depth * 3 - 3..depth * 3, "|- ");
                        my_prefix.replace_range(depth * 3..depth * 3 + key.len(), key);
                        print!("\n{}", my_prefix);
                        self.print(value, my_prefix, icon, depth + 1);
                    }
                }
            }
            _ => {
                // print!(": {}", value);
            }
        }
    }
}

struct PrinterFactory {
    printers: HashMap<String, Box<dyn Printer>>,
}

impl PrinterFactory {
    fn new() -> Self {
        let mut printers: HashMap<String, Box<dyn Printer>> = HashMap::new();
        printers.insert("tree".to_string(), Box::new(TreePrinter));
        printers.insert("rectangle".to_string(), Box::new(RectanglePrinter));
        Self { printers }
    }

    fn get_printer(&self, style: &str) -> Option<&Box<dyn Printer>> {
        self.printers.get(style)
    }
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let file = std::fs::File::open(&opt.file).unwrap();
    let reader = std::io::BufReader::new(file);

    let value: serde_json::Value = serde_json::from_reader(reader).unwrap();

    let icon = Icon::new(&opt.icon);
    let factory = PrinterFactory::new();
    if let Some(printer) = factory.get_printer(&opt.style) {
        printer.print(&value, "".to_string(), &icon, 1);
    } else {
        println!("Unsupported style: {}", opt.style);
    }
}
