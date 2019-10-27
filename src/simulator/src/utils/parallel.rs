pub struct ParallelUtils;

impl ParallelUtils {
    pub fn calculate_chunk_size(data_len: usize, cpu_count: usize) -> usize {
        data_len / cpu_count
    }
}