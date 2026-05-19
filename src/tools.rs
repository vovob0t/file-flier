use std::fmt::Display;

#[derive(Debug)]
pub enum SizeMetric {
    B,
    KB,
    MB,
    GB,
    TB,
    PB,
}

const BYTES: f64 = 1.0;
const KB: f64 = BYTES * 1000.0;
const MB: f64 = KB * 1000.0;
const GB: f64 = MB * 1000.0;
const TB: f64 = GB * 1000.0;
const PB: f64 = TB * 1000.0;

#[derive(Debug)]
pub struct FileSize {
    size: f64,
    metric: SizeMetric,
}

#[derive(Debug)]
pub struct FileNode {
    pub size: FileSize,
    pub name: String,
    pub is_dir: bool,
    pub children: Vec<FileNode>,
}
impl FileNode {
    pub fn new(size: FileSize, name: String, is_dir: bool, children: Vec<FileNode>) -> Self {
        Self {
            size,
            name,
            is_dir,
            children,
        }
    }
}

impl FileSize {
    pub fn size_metric_to_bytes(&self) -> u64 {
        match self.metric {
            SizeMetric::B => self.size as u64,
            SizeMetric::KB => (self.size * KB) as u64,
            SizeMetric::MB => (self.size * MB) as u64,
            SizeMetric::GB => (self.size * GB) as u64,
            SizeMetric::TB => (self.size * TB) as u64,
            SizeMetric::PB => (self.size * PB) as u64,
        }
    }

    pub fn bytes_to_size_metric(size: u64) -> FileSize {
        match size {
            s if s < 100 => FileSize {
                size: s as f64,
                metric: SizeMetric::B,
            },
            s if s < 400_000 => FileSize {
                size: s as f64 / KB,
                metric: SizeMetric::KB,
            },
            s if s < 1_000_000_000 => FileSize {
                size: s as f64 / MB,
                metric: SizeMetric::MB,
            },
            s if s < 1_000_000_000_000 => FileSize {
                size: s as f64 / GB,
                metric: SizeMetric::GB,
            },
            s if s < 1_000_000_000_000_000 => FileSize {
                size: s as f64 / TB,
                metric: SizeMetric::TB,
            },
            s if s < 1_000_000_000_000_000_000 => FileSize {
                size: s as f64 / PB,
                metric: SizeMetric::PB,
            },
            s => FileSize {
                size: s as f64,
                metric: SizeMetric::B,
            },
        }
    }
}

impl Display for FileSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2} {:?}", self.size, self.metric)
    }
}
