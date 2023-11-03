use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufRead, BufReader};

// FAILED: 99749
// FAILED: 99987

#[derive(Debug)]
enum Node {
    Directory(DirectoryNode),
    File(FileNode),
}
impl Node {
    fn get_id(&self) -> u32 {
        match &self {
            Node::Directory(dir) => dir.id,
            Node::File(f) => f.id,
        }
    }
    fn get_parent_node_id(&self) -> u32 {
        match &self {
            Node::Directory(dir) => dir.parent,
            Node::File(f) => f.parent,
        }
    }
    fn add_node_to_dir(&mut self, node: u32) {
        match self {
            Node::Directory(dir) => {
                dir.list.push(node);
            }
            Node::File(_) => panic!("Can't reach here"),
        }
    }
    fn add_size(&mut self, size: u64) {
        match self {
            Node::Directory(dir) => dir.size += size,
            Node::File(_) => panic!("Can't reach here"),
        }
    }
    fn get_size(&self) -> u64 {
        match &self {
            Node::Directory(dir) => dir.size,
            Node::File(f) => f.size,
        }
    }
}
impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Node::Directory(dir) => {
                write!(
                    f,
                    "DIR, name:{}, size:{}, id:{}, parent:{}",
                    dir.name, dir.size, dir.id, dir.parent
                )
            }
            Node::File(file) => {
                write!(
                    f,
                    "FILE, name:{}, size:{}, id:{}, parent:{}",
                    file.name, file.size, file.id, file.parent
                )
            }
        }
    }
}

#[derive(Debug)]
struct DirectoryNode {
    /// Node ID
    id: u32,

    /// Directory name
    name: String,

    /// Node IDs of files and directories in this directory
    list: Vec<u32>,

    /// Node ID of parent directory
    parent: u32,

    /// Total size
    size: u64,
}

#[derive(Debug, Default)]
struct FileNode {
    /// Node ID
    id: u32,

    /// File name
    name: String,

    /// File size
    size: u64,

    /// Node ID of including directory
    parent: u32,
}

#[derive(Debug, Default)]
struct NodeList {
    node: HashMap<u32, Node>,
    num_files: i32,
    num_dirs: i32,
}

impl NodeList {
    fn insert_new_node(&mut self, node: Node) {
        println!("New node added, {}", node);
        match &node {
            Node::Directory(_) => self.num_dirs += 1,
            Node::File(_) => self.num_files += 1,
        };
        self.node.insert(node.get_id(), node);
    }

    fn get_node_mut_ref(&mut self, id: u32) -> &mut Node {
        self.node.get_mut(&id).unwrap()
    }

    fn get_node_ref(&self, id: u32) -> &Node {
        self.node.get(&id).unwrap()
    }

    fn print(&self, id: u32, depth: usize) {
        let node = self.node.get(&id).unwrap();

        match node {
            Node::Directory(dir) => {
                let indent = std::iter::repeat("  ").take(depth).collect::<String>();
                println!("{}, {}", indent, node);

                for n in &dir.list {
                    self.print(*n, depth + 1);
                }
            }
            Node::File(_) => {
                // do nothing
            }
        }
    }

    fn total_nodes(&self) {
        println!(
            "Total nodes, dir:{}, files:{}",
            self.num_dirs, self.num_files
        );
    }
}

struct IdAllocator {
    id: u32,
}
impl IdAllocator {
    fn new() -> Self {
        Self { id: 0 }
    }
    fn allocate(&mut self) -> u32 {
        self.id += 1;
        self.id
    }
}

fn test_input() -> Vec<String> {
    let input = r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    input.lines().map(|l| l.to_string()).collect()
}
fn main() {
    let lines = get_day7_input();
    //let lines = test_input();

    let (root_node_id, nodes) = assess_the_situation_and_return_root_node(lines);
    nodes.print(root_node_id, 0);
    nodes.total_nodes();
    println!();

    let target_num = 100000;
    //my_first_try(root_node_id, &nodes, target_num);
    my_second_try(root_node_id, &nodes, target_num);
}

fn my_first_try(root_node_id: u32, nodes: &NodeList, target_num: u64) {
    // target_num 이하의 sums 를 모두 찾은 뒤 이 중 max 구함 => 결과 = 99749
    // 문제점: greedy 하게 찾아서는 max를 찾을 수 없다! (5+4 < 5+3+2)
    let sizes = get_dir_sizes(root_node_id, &nodes);
    let mut sizes: Vec<u64> = sizes
        .into_iter()
        .filter(|n| *n <= target_num && *n != 0)
        .collect();
    sizes.sort_by(|a, b| b.cmp(a));
    println!("Directory sizes: {:?}", sizes);
    println!();

    let mut sums = Vec::new();
    while sizes.len() != 0 {
        let mut sum = 0;
        for num in &sizes {
            if sum + num <= target_num {
                sum += num;
            }
        }

        sums.push(sum);
        sizes.remove(0);
    }

    println!("sums: {:?}", sums);
    println!("sum: {}", sums.iter().max().unwrap());
}

fn my_second_try(root_node_id: u32, nodes: &NodeList, target_num: u64) {
    // 모든 합을 다 찾은 뒤 이 중 max 구함 => 결과 = 99987
    // 문제점: 더한 값이 같은 경우를 구분할 수 없다...! (10 + 10 = 20 이 되는 최대치를 찾을 수 없다)
    let sizes = get_dir_sizes(root_node_id, &nodes);
    let mut sizes: Vec<u64> = sizes
        .into_iter()
        .filter(|n| *n <= target_num && *n != 0)
        .collect();
    sizes.sort_by(|a, b| b.cmp(a));

    println!("Directory sizes: {:?}", sizes);
    println!();

    let mut last_sums: HashSet<u64> = sizes.into_iter().collect();
    for _ in 0..last_sums.len() - 1 {
        let mut sums = HashSet::new();
        for current in &last_sums {
            let mut sum = current.clone();
            for num in &last_sums {
                if sum + num <= target_num {
                    sum += num;
                }
                sums.insert(sum);
            }
        }

        last_sums = last_sums.union(&sums).cloned().collect();
    }

    println!("sums: {:?}", last_sums);
    println!("sum: {}", last_sums.iter().max().unwrap());
}

fn get_dir_sizes(node_id: u32, nodes: &NodeList) -> Vec<u64> {
    let mut sizes = Vec::new();

    let node = nodes.get_node_ref(node_id);
    if let Node::Directory(dir) = node {
        sizes.push(node.get_size());
        for id in &dir.list {
            sizes.append(&mut get_dir_sizes(*id, nodes));
        }
    }

    return sizes;
}

fn get_day7_input() -> Vec<String> {
    let file = File::open("inputs/day7.txt").unwrap();
    let lines = BufReader::new(file).lines();
    lines.map(|s| s.unwrap()).collect()
}

fn create_root_node(id: u32) -> Node {
    Node::Directory(DirectoryNode {
        id,
        name: "/".to_string(),
        list: Vec::new(),
        parent: 0,
        size: 0,
    })
}

fn assess_the_situation_and_return_root_node(lines: Vec<String>) -> (u32, NodeList) {
    let mut nodes = NodeList::default();
    let mut id_allocator = IdAllocator::new();

    let root_node = create_root_node(id_allocator.allocate());
    let root_node_id = root_node.get_id();
    nodes.insert_new_node(root_node);

    let mut current_node_id = root_node_id;
    let mut waiting_for_output = false;

    for line in lines {
        if line.starts_with("$") {
            // command executed
            waiting_for_output = false;

            let args: Vec<&str> = line.split_whitespace().collect();
            if args[1] == "cd" {
                if args[2] == ".." {
                    let node = nodes.get_node_ref(current_node_id);
                    current_node_id = node.get_parent_node_id();
                } else if args[2] == "/" {
                    current_node_id = root_node_id;
                } else {
                    let dest_node = find_directory(&nodes, current_node_id, args[2]);
                    current_node_id = dest_node;
                }

                println!(
                    "[Current directory] {:?}",
                    nodes.get_node_ref(current_node_id)
                );
            } else if args[1] == "ls" {
                waiting_for_output = true;
            } else {
                panic!("Received invalid command {}", line);
            }
        } else {
            // ls result
            assert!(waiting_for_output);

            let args: Vec<&str> = line.split_whitespace().collect();
            if args[0] == "dir" {
                let node = Node::Directory(DirectoryNode {
                    id: id_allocator.allocate(),
                    name: args[1].to_string(),
                    list: Vec::new(),
                    parent: current_node_id,
                    size: 0,
                });
                let parent = nodes.get_node_mut_ref(current_node_id);
                parent.add_node_to_dir(node.get_id());

                nodes.insert_new_node(node);
            } else {
                let node = Node::File(FileNode {
                    id: id_allocator.allocate(),
                    name: args[1].to_string(),
                    size: args[0].parse::<u64>().unwrap(),
                    parent: current_node_id,
                });
                let parent = nodes.get_node_mut_ref(current_node_id);
                parent.add_node_to_dir(node.get_id());

                let mut parent_node_id = current_node_id;
                while parent_node_id != 0 {
                    let parent_node = nodes.get_node_mut_ref(parent_node_id);
                    parent_node.add_size(node.get_size());
                    parent_node_id = parent_node.get_parent_node_id();
                }

                nodes.insert_new_node(node);
            };
        }
    }
    (root_node_id, nodes)
}

fn find_directory(node_list: &NodeList, current: u32, name: &str) -> u32 {
    let current_node = node_list.get_node_ref(current);
    if let Node::Directory(dir) = current_node {
        for id in &dir.list {
            let inner_node = node_list.get_node_ref(*id);
            if let Node::Directory(dir_node) = inner_node {
                if dir_node.name == name {
                    return *id;
                }
            }
        }
    }

    panic!("Can't reach here");
}
