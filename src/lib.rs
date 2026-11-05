#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyscallSpec {
    pub number: u32,
    pub name: String,
    pub arity: u8,
}

#[derive(Debug, Default)]
pub struct DeveloperGuideRegistry {
    syscalls: Vec<SyscallSpec>,
}

impl DeveloperGuideRegistry {
    pub fn new() -> Self { Self { syscalls: Vec::new() } }
    pub fn register_syscall(&mut self, number: u32, name: &str, arity: u8) {
        self.syscalls.push(SyscallSpec { number, name: name.to_string(), arity });
    }
    pub fn get_by_name(&self, name: &str) -> Option<&SyscallSpec> {
        self.syscalls.iter().find(|s| s.name == name)
    }
    pub fn total_syscalls(&self) -> usize { self.syscalls.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_developer_guide_registry_aaa() {
        // Arrange
        let mut guide = DeveloperGuideRegistry::new();
        // Act
        guide.register_syscall(1, "sys_exit", 1);
        guide.register_syscall(2, "sys_fork", 0);
        guide.register_syscall(3, "sys_read", 3);
        // Assert
        assert_eq!(guide.total_syscalls(), 3);
        let spec = guide.get_by_name("sys_read").expect("sys_read must exist");
        assert_eq!(spec.number, 3);
        assert_eq!(spec.arity, 3);
    }
}
