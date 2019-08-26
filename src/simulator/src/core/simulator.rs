pub struct FootballSimulator{
    thread_count: i32

}

impl FootballSimulator{
    pub fn new(thread_count: i32) -> Self{
        Self{
            thread_count: thread_count
        }
    }
}