pub struct ParallelUtils;

impl ParallelUtils {
    pub fn get_chunk_size(data_len: usize, cpu_count: usize) -> usize {
        if data_len < cpu_count {
            return 1;
        }

        data_len / cpu_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_chunk_size_is_correct() {       
        assert_eq!(2, ParallelUtils.get_chunk_size(16, 8));
        assert_eq!(1, ParallelUtils.get_chunk_size(1, 2));
    }
}
