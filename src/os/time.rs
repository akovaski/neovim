pub type Timestamp = u64;
extern "C" {
    pub fn os_hrtime() -> u64;
}
