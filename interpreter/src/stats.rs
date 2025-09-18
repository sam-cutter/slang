use std::{
    fs,
    time::{Duration, Instant},
};

pub struct Logger {
    start: Instant,
    entries: Vec<Entry>,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            entries: Vec::new(),
        }
    }

    pub fn new_entry(
        &mut self,
        heap_objects_count: usize,
        stack_frames_count: usize,
        heap_size: usize,
        stack_size: usize,
    ) {
        // TODO: calculate actual memory usage
        let interpreter_memory_usage = 0;

        self.entries.push(Entry {
            elapsed: self.start.elapsed(),
            heap_objects_count,
            stack_frames_count,
            heap_size,
            stack_size,
            interpreter_memory_usage,
        });
    }

    pub fn write_to_csv(self, filename: &str) {
        let mut contents = String::from(
            "elapsed,heap_objects_count,stack_frames_count,heap_size,stack_size,interpreter_memory_usage\n",
        );

        for entry in self.entries {
            contents.push_str(
                format!(
                    "{},{},{},{},{},{}\n",
                    entry.elapsed.as_secs_f64(),
                    entry.heap_objects_count,
                    entry.stack_frames_count,
                    entry.heap_size,
                    entry.stack_size,
                    entry.interpreter_memory_usage
                )
                .as_str(),
            );
        }

        let _ = fs::write(filename, contents);
    }
}

struct Entry {
    elapsed: Duration,
    heap_objects_count: usize,
    stack_frames_count: usize,
    heap_size: usize,
    stack_size: usize,
    interpreter_memory_usage: usize,
}
