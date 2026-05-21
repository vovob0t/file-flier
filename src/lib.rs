use std::{cell::RefCell, os::unix::fs::MetadataExt, path::Path, rc::Rc};

pub mod tools;
use tools::{FileNode, FileSize};

pub mod config;
use config::{Config, SortDirection, SortType};

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    list_directory_items(config.path, config.sort_type)?;
    Ok(())
}

pub fn list_directory_items<T: AsRef<Path>>(
    path: T,
    sort_type: SortType,
) -> Result<FileNode, Box<dyn std::error::Error>> {
    let path = path.as_ref();

    // if !path.exists() {
    //     return Err(Error::new(
    //         io::ErrorKind::InvalidFilename,
    //         format!("Path {path:?} doesn't exist"),
    //     ));
    // };

    if path.is_file() {
        let file_node = create_file_node_from_path(path)?;
        println!(
            "File {:?} with size {}, is file - {}",
            file_node.name, file_node.size, !file_node.is_dir
        );
        return Ok(file_node);
    };

    let path = path.to_str().unwrap().replace("~", "/home/vovobot");

    let mut dir_tree = create_dir_tree_from_path(path.as_ref())?;
    sort_file_tree(&mut dir_tree, &sort_type);

    println!("|{:?} - {}|", dir_tree.name, dir_tree.size);
    println!("================================");

    for dir in &dir_tree.children {
        println!("{} - {}", dir.borrow().name, dir.borrow().size)
    }

    Ok(dir_tree)
}

pub fn sort_file_tree(file_tree: &mut FileNode, sort_type: &SortType) {
    match sort_type {
        SortType::Size(x) => match x {
            SortDirection::Up => {
                file_tree.children.sort_by(|a, b| {
                    a.borrow()
                        .size
                        .size_metric_to_bytes()
                        .cmp(&b.borrow().size.size_metric_to_bytes())
                });
            }
            SortDirection::Down => {
                file_tree.children.sort_by(|a, b| {
                    b.borrow()
                        .size
                        .size_metric_to_bytes()
                        .cmp(&a.borrow().size.size_metric_to_bytes())
                });
            }
        },
        SortType::Alphabetical(x) => match x {
            SortDirection::Up => {
                file_tree
                    .children
                    .sort_by(|a, b| a.borrow().name.cmp(&b.borrow().name));
            }
            SortDirection::Down => {
                file_tree
                    .children
                    .sort_by(|a, b| b.borrow().name.cmp(&a.borrow().name));
            }
        },
        _ => (),
    }
}

pub fn create_dir_tree_from_path(dir: &Path) -> Result<FileNode, Box<dyn std::error::Error>> {
    let mut bytes_count: u64 = 0;
    let mut children: Vec<Rc<RefCell<FileNode>>> = vec![];
    let mut is_dir: bool = false;
    let name = dir.to_str().unwrap().to_string();

    if name.starts_with("/proc") {
        let size = FileSize::bytes_to_size_metric(0);
        return Ok(FileNode::new(size, name, true, children));
    };

    let Ok(dir_entries) = dir.read_dir() else {
        // println!("Couldn't read - {:?}", dir);
        let size = FileSize::bytes_to_size_metric(0);
        return Ok(FileNode::new(size, name, is_dir, children));
    };

    for entry in dir_entries {
        // println!("Rised");
        match entry {
            Ok(entry) => match entry.metadata()?.is_dir() {
                false => {
                    bytes_count += entry.metadata()?.size();

                    let Ok(child_file_node) = create_file_node_from_path(&entry.path()) else {
                        // println!("Couldn't read - {:?}", &entry.path());
                        continue;
                    };

                    children.push(Rc::new(RefCell::new(child_file_node)));
                }
                true => {
                    is_dir = true;

                    let Ok(child_dir_node) = create_dir_tree_from_path(&entry.path()) else {
                        // println!("Couldn't read - {:?}", &entry.path());
                        continue;
                    };

                    bytes_count += child_dir_node.size.size_metric_to_bytes();
                    children.push(Rc::new(RefCell::new(child_dir_node)));
                }
            },
            Err(e) => {
                println!("{:?} - couldn't read because - {}", dir, e);
                continue;
            }
        }
    }

    let size = FileSize::bytes_to_size_metric(bytes_count);

    Ok(FileNode::new(size, name, is_dir, children))
}

pub fn create_file_node_from_path(entry: &Path) -> Result<FileNode, Box<dyn std::error::Error>> {
    let size = entry.metadata()?.size();
    let is_dir = false;
    let size = FileSize::bytes_to_size_metric(size);
    let name = entry.to_str().unwrap().to_string();
    // let children: Vec<FileNode> = vec![];

    Ok(FileNode::new(size, name, is_dir, vec![]))
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::{
        config::{SortDirection, SortType},
        list_directory_items,
    };

    #[test]
    fn check() {
        println!("{:?}", env::current_dir());
        assert!(
            list_directory_items(
                "./src/test_dir/test_file",
                SortType::Natural(SortDirection::Up)
            )
            .is_ok()
        );
    }
}
