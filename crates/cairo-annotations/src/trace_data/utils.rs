use std::collections::HashMap;
use std::ops::{AddAssign, SubAssign};

use super::{ExecutionResources, SyscallUsage, VmExecutionResources};

impl AddAssign<&VmExecutionResources> for VmExecutionResources {
    fn add_assign(&mut self, other: &VmExecutionResources) {
        self.n_steps += other.n_steps;
        self.n_memory_holes += other.n_memory_holes;

        for (key, value) in &other.builtin_instance_counter {
            *self
                .builtin_instance_counter
                .entry(key.clone())
                .or_insert(0) += *value;
        }
    }
}

impl SubAssign<&VmExecutionResources> for VmExecutionResources {
    fn sub_assign(&mut self, other: &VmExecutionResources) {
        self.n_steps = self.n_steps.saturating_sub(other.n_steps);
        self.n_memory_holes = self.n_memory_holes.saturating_sub(other.n_memory_holes);

        for (key, value) in &other.builtin_instance_counter {
            if let Some(self_value) = self.builtin_instance_counter.get_mut(key) {
                *self_value = self_value.saturating_sub(*value);
            }
        }
        // Remove entries where the value is 0
        self.builtin_instance_counter.retain(|_, value| *value > 0);
    }
}

impl AddAssign<&ExecutionResources> for ExecutionResources {
    fn add_assign(&mut self, other: &ExecutionResources) {
        self.vm_resources += &other.vm_resources;
        self.gas_consumed = match (self.gas_consumed, other.gas_consumed) {
            (Some(self_gas), Some(other_gas)) => Some(self_gas + other_gas),
            (Some(self_gas), None) => Some(self_gas),
            (None, Some(other_gas)) => Some(other_gas),
            (None, None) => None,
        };

        if let Some(other_counter) = &other.syscall_counter {
            if let Some(self_counter) = &mut self.syscall_counter {
                for (selector, usage) in other_counter {
                    self_counter
                        .entry(*selector)
                        .and_modify(|existing| {
                            existing.call_count += usage.call_count;
                            existing.linear_factor += usage.linear_factor;
                        })
                        .or_insert_with(|| SyscallUsage {
                            call_count: usage.call_count,
                            linear_factor: usage.linear_factor,
                        });
                }
            } else {
                // If self doesn't have a counter but other does, clone other's counter
                let mut new_counter = HashMap::new();
                for (selector, usage) in other_counter {
                    new_counter.insert(
                        *selector,
                        SyscallUsage {
                            call_count: usage.call_count,
                            linear_factor: usage.linear_factor,
                        },
                    );
                }
                self.syscall_counter = Some(new_counter);
            }
        }
    }
}

impl SubAssign<&ExecutionResources> for ExecutionResources {
    fn sub_assign(&mut self, other: &ExecutionResources) {
        self.vm_resources -= &other.vm_resources;

        if let Some(other_gas) = other.gas_consumed {
            if let Some(self_gas) = &mut self.gas_consumed {
                *self_gas = self_gas.saturating_sub(other_gas);
            }
        }

        if let Some(other_counter) = &other.syscall_counter {
            if let Some(self_counter) = &mut self.syscall_counter {
                for (selector, usage) in other_counter {
                    if let Some(self_usage) = self_counter.get_mut(selector) {
                        self_usage.call_count =
                            self_usage.call_count.saturating_sub(usage.call_count);
                        self_usage.linear_factor =
                            self_usage.linear_factor.saturating_sub(usage.linear_factor);
                    }
                }
                // Remove entries where both values are 0
                self_counter.retain(|_, usage| usage.call_count > 0 || usage.linear_factor > 0);
            }
        }
    }
}
