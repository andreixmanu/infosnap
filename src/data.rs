use std::fmt::Display;

pub struct OsInfo
{
    pub kernel: String,
    pub hostname: String,
    pub type_: String,
    pub uptime: Uptime,
    pub cpu: String,
    // pub cpu_freq : f32,
    pub gpu: String,
    pub local_ip: String,
    // pub public_ip: String,
    pub used_memory: f32,
    pub total_memory: f32,
}

impl Display for OsInfo
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "OS Version: {}\n", self.kernel)?;
        write!(f, "Hostname: {}\n", self.hostname)?;
        write!(f, "OS Type: {}\n", self.type_)?;
        write!(f, "Uptime: {}\n", self.uptime)?;
        write!(f, "CPU : {}\n", self.cpu)?;
        write!(f, "GPU: {}\n", self.gpu)?;
        write!(f, "Local IP: {}\n", self.local_ip)?;
        //write!(f, "Public IP: {}\n", self.public_ip)?;
        write!(f, "Used Memory: {}\n", self.used_memory)?;
        write!(f, "Total Memory: {}\n", self.total_memory)
    }
}

pub struct Uptime {
    pub days: u64,
    pub hours : u64,
    pub minutes : u64,
    pub seconds : u64,
}

impl Display for Uptime
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "Uptime: {} days, {} hours, {} minutes, {} seconds", self.days, self.hours, self.minutes, self.seconds).expect("Uptime: Error in formatting uptime.");
        Ok(())
    }
}