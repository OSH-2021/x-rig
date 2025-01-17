// config.rs - Macros starting with "config" who evaluate to a numeric value.
// This file is adapted from FreeRTOSConfig.h

// TODO: Provide configASSERT() (or not?).
#[macro_export]
macro_rules! configTICK_RATE_HZ {
    () => {
        1000 as port::TickType
    };
}

#[macro_export]
macro_rules! configMINIMAL_STACK_SIZE {
    () => {
        64
    };
}

#[macro_export]
macro_rules! configTOTAL_HEAP_SIZE {
    () => {
        64 * 1024 as usize
    };
}

#[macro_export]
macro_rules! configMAX_TASK_NAME_LEN {
    () => {
        16
    };
}

#[macro_export]
macro_rules! configQUEUE_REGISTRY_SIZE {
    () => {
        20
    };
}

#[macro_export]
macro_rules! configMAX_PRIORITIES {
    () => {
        10
    };
}

#[macro_export]
macro_rules! configMAX_ENDPOINTS {
    () => {
        10
    };
}

#[macro_export]
macro_rules! configTIMER_TASK_PRIORITY {
    () => {
        configMAX_PRIORITIES!() - 1
    };
}

#[macro_export]
macro_rules! configTIMER_TASK_STACK_DEPTH {
    () => {
        configMINIMAL_STACK_SIZE * 2
    };
}

#[macro_export]
macro_rules! configEXPECTED_IDLE_TIME_BEFORE_SLEEP {
    () => {
        2
    };
}

#[macro_export]
macro_rules! taskNOTIFICATION_RECEIVED {
    () => {
        2
    };
}

#[macro_export]
macro_rules! taskWAITING_NOTIFICATION {
    () => {
        1
    };
}

#[macro_export]
macro_rules! taskNOT_WAITING_NOTIFICATION {
    () => {
        0
    };
}
