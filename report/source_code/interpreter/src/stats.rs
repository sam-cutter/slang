use std::{
    fs::{self, File},
    io::{self, BufRead},
    path::Path,
    time::{Duration, Instant},
};

pub struct Logger {
    start: Instant,
    entries: Vec<Entry>,
}

fn get_memory_usage() -> Option<usize> {
    let path = Path::new("/proc/self/status");

    if let Ok(file) = File::open(path) {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let line = line;

            if let Ok(line) = line {
                if !line.starts_with("VmRSS:") {
                    continue;
                }

                let parts: Vec<&str> = line.split_whitespace().collect();

                // Get the VmRSS value, which is in kilobytes.
                if let Some(rss_kb) = parts.get(1) {
                    return Some(rss_kb.parse::<usize>().unwrap_or(0) * 1000);
                }
            }
        }
    }

    None
}

impl Logger {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            entries: Vec::new(),
        }
    }

    pub fn new_entry(&mut self, heap_objects_count: usize, stack_frames_count: usize) {
        let memory_usage = get_memory_usage();

        self.entries.push(Entry {
            elapsed: self.start.elapsed(),
            heap_objects_count,
            stack_frames_count,
            memory_usage,
        });
    }

    pub fn write_to_csv(self, source_code_filename: &str) {
        let mut contents =
            String::from("elapsed,heap_objects_count,stack_frames_count,interpreter_memory_usage");

        for entry in self.entries {
            let memory_usage = if let Some(memory_usage) = entry.memory_usage {
                format!("{}", memory_usage)
            } else {
                String::from("unable to calculate")
            };

            contents.push_str(
                format!(
                    "\n{},{},{},{}",
                    entry.elapsed.as_secs_f64(),
                    entry.heap_objects_count,
                    entry.stack_frames_count,
                    memory_usage,
                )
                .as_str(),
            );
        }

        let filename = format!("{}.csv", source_code_filename);

        let _ = fs::write(filename, contents);
    }
}

struct Entry {
    elapsed: Duration,
    heap_objects_count: usize,
    stack_frames_count: usize,
    memory_usage: Option<usize>,
}
