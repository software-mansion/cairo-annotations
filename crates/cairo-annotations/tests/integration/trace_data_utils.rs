use std::collections::HashMap;

use cairo_annotations::trace_data::{
    DeprecatedSyscallSelector, ExecutionResources, SyscallUsage, VmExecutionResources,
};

#[test]
fn test_vm_execution_resources_add() {
    let mut vm1 = VmExecutionResources {
        n_steps: 10,
        n_memory_holes: 5,
        builtin_instance_counter: HashMap::new(),
    };
    vm1.builtin_instance_counter
        .insert("builtin1".to_string(), 3);
    vm1.builtin_instance_counter
        .insert("builtin2".to_string(), 2);

    let mut vm2 = VmExecutionResources {
        n_steps: 20,
        n_memory_holes: 8,
        builtin_instance_counter: HashMap::new(),
    };
    vm2.builtin_instance_counter
        .insert("builtin2".to_string(), 4);
    vm2.builtin_instance_counter
        .insert("builtin3".to_string(), 1);

    vm1 += &vm2;

    assert_eq!(vm1.n_steps, 30);
    assert_eq!(vm1.n_memory_holes, 13);
    assert_eq!(vm1.builtin_instance_counter.get("builtin1"), Some(&3));
    assert_eq!(vm1.builtin_instance_counter.get("builtin2"), Some(&6));
    assert_eq!(vm1.builtin_instance_counter.get("builtin3"), Some(&1));
}

#[test]
fn test_vm_execution_resources_sub() {
    let mut vm1 = VmExecutionResources {
        n_steps: 30,
        n_memory_holes: 15,
        builtin_instance_counter: HashMap::new(),
    };
    vm1.builtin_instance_counter
        .insert("builtin1".to_string(), 5);
    vm1.builtin_instance_counter
        .insert("builtin2".to_string(), 8);
    vm1.builtin_instance_counter
        .insert("builtin3".to_string(), 3);

    let mut vm2 = VmExecutionResources {
        n_steps: 10,
        n_memory_holes: 5,
        builtin_instance_counter: HashMap::new(),
    };
    vm2.builtin_instance_counter
        .insert("builtin1".to_string(), 2);
    vm2.builtin_instance_counter
        .insert("builtin2".to_string(), 8); // This will become 0 and should be removed
    vm2.builtin_instance_counter
        .insert("builtin4".to_string(), 1); // This key doesn't exist in vm1

    vm1 -= &vm2;

    assert_eq!(vm1.n_steps, 20);
    assert_eq!(vm1.n_memory_holes, 10);
    assert_eq!(vm1.builtin_instance_counter.get("builtin1"), Some(&3));
    assert_eq!(vm1.builtin_instance_counter.get("builtin2"), None); // Should be removed as it becomes 0
    assert_eq!(vm1.builtin_instance_counter.get("builtin3"), Some(&3));
    assert_eq!(vm1.builtin_instance_counter.get("builtin4"), None); // Doesn't exist in vm1
}

#[test]
fn test_execution_resources_add() {
    let mut vm1 = VmExecutionResources {
        n_steps: 10,
        n_memory_holes: 5,
        builtin_instance_counter: HashMap::new(),
    };
    vm1.builtin_instance_counter
        .insert("builtin1".to_string(), 3);

    let mut vm2 = VmExecutionResources {
        n_steps: 20,
        n_memory_holes: 8,
        builtin_instance_counter: HashMap::new(),
    };
    vm2.builtin_instance_counter
        .insert("builtin2".to_string(), 4);

    let mut syscall_counter1 = HashMap::new();
    syscall_counter1.insert(
        DeprecatedSyscallSelector::Deploy,
        SyscallUsage {
            call_count: 2,
            linear_factor: 3,
        },
    );

    let mut syscall_counter2 = HashMap::new();
    syscall_counter2.insert(
        DeprecatedSyscallSelector::Deploy,
        SyscallUsage {
            call_count: 1,
            linear_factor: 2,
        },
    );
    syscall_counter2.insert(
        DeprecatedSyscallSelector::EmitEvent,
        SyscallUsage {
            call_count: 3,
            linear_factor: 0,
        },
    );

    let mut er1 = ExecutionResources {
        vm_resources: vm1,
        gas_consumed: Some(100),
        syscall_counter: Some(syscall_counter1),
    };

    let er2 = ExecutionResources {
        vm_resources: vm2,
        gas_consumed: Some(50),
        syscall_counter: Some(syscall_counter2),
    };

    er1 += &er2;

    assert_eq!(er1.vm_resources.n_steps, 30);
    assert_eq!(er1.vm_resources.n_memory_holes, 13);
    assert_eq!(er1.gas_consumed, Some(150));

    let syscall_counter = er1.syscall_counter.unwrap();
    let call_contract_usage = syscall_counter
        .get(&DeprecatedSyscallSelector::Deploy)
        .unwrap();
    assert_eq!(call_contract_usage.call_count, 3);
    assert_eq!(call_contract_usage.linear_factor, 5);

    let emit_event_usage = syscall_counter
        .get(&DeprecatedSyscallSelector::EmitEvent)
        .unwrap();
    assert_eq!(emit_event_usage.call_count, 3);
    assert_eq!(emit_event_usage.linear_factor, 0);
}

#[test]
fn test_execution_resources_sub() {
    let mut vm1 = VmExecutionResources {
        n_steps: 30,
        n_memory_holes: 15,
        builtin_instance_counter: HashMap::new(),
    };
    vm1.builtin_instance_counter
        .insert("builtin1".to_string(), 5);

    let mut vm2 = VmExecutionResources {
        n_steps: 10,
        n_memory_holes: 5,
        builtin_instance_counter: HashMap::new(),
    };
    vm2.builtin_instance_counter
        .insert("builtin1".to_string(), 2);

    let mut syscall_counter1 = HashMap::new();
    syscall_counter1.insert(
        DeprecatedSyscallSelector::Deploy,
        SyscallUsage {
            call_count: 5,
            linear_factor: 7,
        },
    );
    syscall_counter1.insert(
        DeprecatedSyscallSelector::EmitEvent,
        SyscallUsage {
            call_count: 3,
            linear_factor: 0,
        },
    );

    let mut syscall_counter2 = HashMap::new();
    syscall_counter2.insert(
        DeprecatedSyscallSelector::Deploy,
        SyscallUsage {
            call_count: 2,
            linear_factor: 3,
        },
    );
    syscall_counter2.insert(
        DeprecatedSyscallSelector::EmitEvent,
        SyscallUsage {
            call_count: 3,
            linear_factor: 0,
        },
    ); // This will become 0 and should be removed

    let mut er1 = ExecutionResources {
        vm_resources: vm1,
        gas_consumed: Some(100),
        syscall_counter: Some(syscall_counter1),
    };

    let er2 = ExecutionResources {
        vm_resources: vm2,
        gas_consumed: Some(30),
        syscall_counter: Some(syscall_counter2),
    };

    er1 -= &er2;

    assert_eq!(er1.vm_resources.n_steps, 20);
    assert_eq!(er1.vm_resources.n_memory_holes, 10);
    assert_eq!(er1.gas_consumed, Some(70));

    let syscall_counter = er1.syscall_counter.unwrap();
    let call_contract_usage = syscall_counter
        .get(&DeprecatedSyscallSelector::Deploy)
        .unwrap();
    assert_eq!(call_contract_usage.call_count, 3);
    assert_eq!(call_contract_usage.linear_factor, 4);

    // EmitEvent should be removed as both values are 0
    assert!(!syscall_counter.contains_key(&DeprecatedSyscallSelector::EmitEvent));
}
