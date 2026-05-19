use std::{os::unix::fs::MetadataExt, path::Path};

mod tools;
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
    sort_file_tree(&mut dir_tree, sort_type);

    println!("|{:?} - {}|", dir_tree.name, dir_tree.size);
    println!("================================");

    for dir in &dir_tree.children {
        println!("{} - {}", dir.name, dir.size)
    }

    Ok(dir_tree)
}

pub fn sort_file_tree(file_tree: &mut FileNode, sort_type: SortType) {
    match sort_type {
        SortType::Size(x) => match x {
            SortDirection::Up => {
                file_tree.children.sort_by(|a, b| {
                    a.size
                        .size_metric_to_bytes()
                        .cmp(&b.size.size_metric_to_bytes())
                });
            }
            SortDirection::Down => {
                file_tree.children.sort_by(|a, b| {
                    b.size
                        .size_metric_to_bytes()
                        .cmp(&a.size.size_metric_to_bytes())
                });
            }
        },
        SortType::Alphabetical(x) => match x {
            SortDirection::Up => {
                file_tree.children.sort_by(|a, b| a.name.cmp(&b.name));
            }
            SortDirection::Down => {
                file_tree.children.sort_by(|a, b| b.name.cmp(&a.name));
            }
        },
        _ => (),
    }
}

// pub fn count_directory_size(dir: &DirEntry) -> Result<u64, Box<dyn std::error::Error>> {
//     let mut bytes_count: u64 = 0;
//
//     for entry in dir.path().read_dir()? {
//         bytes_count += match entry {
//             Ok(entry) => match entry.metadata()?.is_dir() {
//                 false => entry.metadata()?.size(),
//                 true => count_directory_size(&entry)?,
//             },
//             _ => continue,
//         }
//     }
//
//     Ok(bytes_count)
// }
pub fn create_dir_tree_from_path(dir: &Path) -> Result<FileNode, Box<dyn std::error::Error>> {
    let mut bytes_count: u64 = 0;
    let mut children: Vec<FileNode> = vec![];
    let mut is_dir: bool = false;
    let name = dir.to_str().unwrap().to_string();

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

                    children.push(child_file_node);
                }
                true => {
                    is_dir = true;

                    let Ok(child_node) = create_dir_tree_from_path(&entry.path()) else {
                        // println!("Couldn't read - {:?}", &entry.path());
                        continue;
                    };

                    bytes_count += child_node.size.size_metric_to_bytes();
                    children.push(child_node);
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
    let children: Vec<FileNode> = vec![];

    Ok(FileNode::new(size, name, is_dir, children))
}

// pub fn create_dir_tree(dir: &DirEntry) -> Result<FileNode, Box<dyn std::error::Error>> {
//     let mut bytes_count: u64 = 0;
//     let mut children: Vec<FileNode> = vec![];
//     let mut is_dir: bool = false;
//     let name = dir.path().to_str().unwrap().to_string();
//
//     for entry in dir.path().read_dir()? {
//         match entry {
//             Ok(entry) => match entry.metadata()?.is_dir() {
//                 false => {
//                     bytes_count += entry.metadata()?.size();
//                     let child_file_node = create_file_node(&entry)?;
//                     children.push(child_file_node);
//                 }
//                 true => {
//                     is_dir = true;
//                     // name = entry.path().to_str().unwrap().to_string();
//                     // println!("{}", name);
//                     let child_node = create_dir_tree(&entry)?;
//                     bytes_count += child_node.size.size_metric_to_bytes();
//                     children.push(child_node);
//                 }
//             },
//             Err(e) => {
//                 println!("{:?} - couldn't read because - {}", dir.path(), e);
//                 continue;
//             }
//         }
//     }
//     let size = FileSize::bytes_to_size_metric(bytes_count);
//
//     Ok(FileNode::new(size, name, is_dir, children))
// }
//
// pub fn create_file_node(entry: &DirEntry) -> Result<FileNode, Box<dyn std::error::Error>> {
//     let size = entry.metadata()?.size();
//     let is_dir = false;
//     let size = FileSize::bytes_to_size_metric(size);
//     let name = entry.path().to_str().unwrap().to_string();
//     let children: Vec<FileNode> = vec![];
//
//     Ok(FileNode::new(size, name, is_dir, children))
// }

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
