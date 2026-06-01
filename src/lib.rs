use parking_lot::RwLock;
use std::{io, os::unix::fs::MetadataExt, path::Path, sync::Arc, thread};

pub mod tools;
use tools::{FileNode, FileSize};

pub mod config;
use config::{Config, SortDirection, SortType};

const TMP_VIRTUA_DIRS: [&str; 4] = ["/proc", "/run", "/dev", "/sys"];

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
        println!("{} - {}", dir.read().name, dir.read().size)
    }

    Ok(dir_tree)
}

pub fn sort_file_tree(file_tree: &mut FileNode, sort_type: &SortType) {
    match sort_type {
        SortType::Size(x) => match x {
            SortDirection::Up => {
                file_tree.children.sort_by(|a, b| {
                    a.read()
                        .size
                        .size_metric_to_bytes()
                        .cmp(&b.read().size.size_metric_to_bytes())
                });
            }
            SortDirection::Down => {
                file_tree.children.sort_by(|a, b| {
                    b.read()
                        .size
                        .size_metric_to_bytes()
                        .cmp(&a.read().size.size_metric_to_bytes())
                });
            }
        },
        SortType::Alphabetical(x) => match x {
            SortDirection::Up => {
                file_tree
                    .children
                    .sort_by(|a, b| a.read().name.cmp(&b.read().name));
            }
            SortDirection::Down => {
                file_tree
                    .children
                    .sort_by(|a, b| b.read().name.cmp(&a.read().name));
            }
        },
        _ => (),
    }
}

pub fn create_dir_tree_from_path(dir: &Path) -> Result<FileNode, io::Error> {
    let mut bytes_count: u64 = 0;

    let mut children: Vec<Arc<RwLock<FileNode>>> = vec![];

    let is_dir: bool = true;
    let name = dir.to_str().unwrap_or("NOT_AUTHORIZED_TO_READ").to_string();

    if TMP_VIRTUA_DIRS.contains(&name.as_str()) {
        // if name.starts_with("/proc") || name.starts_with("/tmp") {
        let size = FileSize::bytes_to_size_metric(0);
        return Ok(FileNode::new(
            size,
            name + " - system directory, not being scanned",
            true,
            children,
        ));
    };

    let Ok(dir_entries) = dir.read_dir() else {
        // println!("Couldn't read - {:?}", dir);
        let size = FileSize::bytes_to_size_metric(0);
        return Ok(FileNode::new(size, name, is_dir, children));
    };

    let bytes_mutex = Arc::new(RwLock::new(0_u64));

    let children_mutex: Arc<RwLock<Vec<Arc<RwLock<FileNode>>>>> = Arc::new(RwLock::new(vec![]));
    let mut thread_pool = vec![];

    for entry in dir_entries {
        match entry {
            Ok(entry) => match entry.metadata()?.is_dir() {
                false => {
                    // let path = entry.path();
                    // let size = entry.metadata()?.size();
                    // let bytes_ref = Arc::clone(&bytes_mutex);
                    // let children_ref = Arc::clone(&children_mutex);
                    //
                    // let handle = thread::spawn(move || {
                    //     *bytes_ref.write() += size;
                    //
                    //     let Ok(children_file_node) = create_file_node_from_path(&path) else {
                    //         return;
                    //     };
                    //
                    //     children_ref
                    //         .write()
                    //         .push(Arc::new(RwLock::new(children_file_node)));
                    // });
                    //
                    // thread_pool.push(handle);

                    bytes_count += entry.metadata()?.size();

                    let Ok(child_file_node) = create_file_node_from_path(&entry.path()) else {
                        eprintln!("Couldn't read - {:?}", &entry.path());
                        continue;
                    };

                    children
                        // .write()
                        .push(Arc::new(RwLock::new(child_file_node)));
                }
                true => {
                    let path = entry.path();
                    let bytes_ref = Arc::clone(&bytes_mutex);
                    let children_ref = Arc::clone(&children_mutex);

                    // let bytes_tx1 = bytes_tx.clone();
                    let handle = thread::spawn(move || {
                        // eprintln!("Sosalka");
                        let Ok(file_node) = create_dir_tree_from_path(&path) else {
                            eprintln!("Couldn't read directory - {}", path.display());
                            return;
                        };

                        *bytes_ref.write() += file_node.size.size_metric_to_bytes();

                        // bytes_tx1
                        //     .send(file_node.as_ref().unwrap().size.size_metric_to_bytes())
                        //     .unwrap();

                        children_ref.write().push(Arc::new(RwLock::new(file_node)));
                        // tx.send(file_node).unwrap();
                        //
                    });

                    thread_pool.push(handle);
                    // let Ok(child_dir_node) = create_dir_tree_from_path(&entry.path()) else {
                    //     // println!("Couldn't read - {:?}", &entry.path());
                    //     continue;
                    // };

                    // bytes_count += child_dir_node.size.size_metric_to_bytes();
                    // children.push(Arc::new(RwLock::new(child_dir_node)));
                }
            },
            Err(e) => {
                eprintln!("{:?} - couldn't read because - {}", dir, e);
                continue;
            }
        }
    }

    // let mut size: u64 = 0;
    // for bytes in bytes_rx {
    //     size += bytes;
    // }

    for handle in thread_pool {
        if let Err(err) = handle.join() {
            eprintln!("Couldn't join handle\n{err:#?}");
        };
    }

    let size = FileSize::bytes_to_size_metric(bytes_count + *bytes_mutex.read());

    children.append(&mut children_mutex.write());

    Ok(FileNode::new(size, name, is_dir, children))
}

pub fn create_file_node_from_path(entry: &Path) -> Result<FileNode, Box<dyn std::error::Error>> {
    let size = entry.metadata()?.size();
    let is_dir = false;
    let size = FileSize::bytes_to_size_metric(size);
    let name = entry
        .to_str()
        .unwrap_or("NOT_AUTHORIZED_TO_READ")
        .to_string();
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
