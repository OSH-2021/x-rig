// Depress some warnings caused by our C bindings.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![feature(test)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate simplelog;
extern crate test;

mod bindings; // This file is generated by bindgen and doesn't show up in the git repo.
pub mod config;
pub mod ffi;
pub mod list;
pub mod port;
pub mod projdefs;
pub mod task_control;
pub mod task_global;
mod trace;
// mod task_api;
pub mod kernel;
pub mod queue;
pub mod queue_api;
mod queue_h;
mod task_queue;
//mod mutex;
pub mod semaphore;
pub mod task_timemanager;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    /*
    // Note! This test SHOULD FAIL, showing something like this:
    // test tests::test_vPortYield ... error: process didn't exit successfully: `/rust_freertos/target/debug/deps/rust_freertos-f3432ee83a2dce9a` (signal: 11, SIGSEGV: invalid memory reference)
    #[test]
    fn test_portYIELD() {
        portYIELD!()
    }
    */

    /*
    // Note! This test SHOULD FAIL as well.
    // BUT on my Mac it just doesn't stop running. Weird.
    use port;
    #[test]
    fn test_port_start_scheduler() {
        port::port_start_scheduler();
    }
    */

    use port;
    use task_control;
    // #[bench]
    // fn test_mutex(b: &mut Bencher) {
    //     use semaphore::Semaphore;
    //     use simplelog::*;

    //     let task0 = move || {
    //         let mut tmp = 1;
    //         for i in 1..11 {
    //             tmp = tmp * i;
    //         }
    //         kernel::task_end_scheduler();
    //     };

    //     let task1 = move || {
    //         let mut tmp = 1;
    //         for i in 1..11 {
    //             tmp = tmp * i;
    //         }
    //         kernel::task_end_scheduler();
    //     };

    //     let task2 = move || {
    //         let mut tmp = 1;
    //         for i in 1..11 {
    //             tmp = tmp * i;
    //         }
    //         kernel::task_end_scheduler();
    //     };

    //     let task3 = move || {
    //         let mut tmp = 1;
    //         for i in 1..11 {
    //             tmp = tmp * i;
    //         }
    //         kernel::task_end_scheduler();
    //     };

    //     let task4 = move || {
    //         let mut tmp = 1;
    //         for i in 1..11 {
    //             tmp = tmp * i;
    //         }
    //         kernel::task_end_scheduler();
    //     };

    //     let task5 = move || {
    //         let mut tmp = 1;
    //         for i in 1..11 {
    //             tmp = tmp * i;
    //         }
    //         kernel::task_end_scheduler();
    //     };

    //     let task6 = move || {
    //         let mut tmp = 1;
    //         for i in 1..11 {
    //             tmp = tmp * i;
    //         }
    //         kernel::task_end_scheduler();
    //     };

    //     let task7 = move || {
    //         let mut tmp = 1;
    //         for i in 1..11 {
    //             tmp = tmp * i;
    //         }
    //         kernel::task_end_scheduler();
    //     };


    //     b.iter(|| {let Task0 = task_control::TCB::new()
    //         .name("Task0")
    //         .priority(3)
    //         .initialise(task0);});
    //     b.iter(|| {let Task1 = task_control::TCB::new()
    //         .name("Task0")
    //         .priority(3)
    //         .initialise(task1);});
    //     b.iter(|| {let Task2 = task_control::TCB::new()
    //         .name("Task0")
    //         .priority(3)
    //         .initialise(task2);});
    //     b.iter(|| {let Task3 = task_control::TCB::new()
    //         .name("Task0")
    //         .priority(3)
    //         .initialise(task3);});
    //     b.iter(|| {let Task4 = task_control::TCB::new()
    //         .name("Task0")
    //         .priority(3)
    //         .initialise(task4);});
    //     b.iter(|| {let Task5 = task_control::TCB::new()
    //         .name("Task0")
    //         .priority(3)
    //         .initialise(task5);});
    //     b.iter(|| {let task6 = task_control::TCB::new()
    //         .name("Task0")
    //         .priority(3)
    //         .initialise(task6);});
    //     b.iter(|| {let Task7 = task_control::TCB::new()
    //         .name("Task0")
    //         .priority(3)
    //         .initialise(task7);});
    //     kernel::task_start_scheduler();
    // }

    use std::sync::Arc;
    #[test]
    fn test_recursive_mutex() {
        use semaphore::Semaphore;
        use simplelog::*;

        let _ = TermLogger::init(LevelFilter::Trace, Config::default());
        let recursive_mutex = Semaphore::create_recursive_mutex();

        let mutex_holder = move || {
            for i in 1..11 {
                trace!("Call down_recursive");
                recursive_mutex.down_recursive(pdMS_TO_TICKS!(0));
                assert!(recursive_mutex.get_recursive_count() == i);
            }

            for j in 1..11 {
                recursive_mutex.up_recursive();
                assert!(recursive_mutex.get_recursive_count() == 10-j);
            }
            kernel::task_end_scheduler();
        };

        let recursive_mutex_holder = task_control::TCB::new()
                                .name("Recursive_mutex_holder")
                                .priority(3)
                                .initialise(mutex_holder);

        kernel::task_start_scheduler();
    }

    #[test]
    fn test_mutex() {
        use semaphore::Semaphore;
        use simplelog::*;

        let _ = TermLogger::init(LevelFilter::Trace, Config::default());
        let mutex0 = Arc::new(Semaphore::new_mutex());
        let mutex1 = Arc::clone(&mutex0);

        let task0 = move || {
            task_timemanager::task_delay(pdMS_TO_TICKS!(1));
            loop {
                match mutex0.semaphore_down(pdMS_TO_TICKS!(0)) {
                    Ok(_) => {
                        for i in 1..11 {
                            trace!("Task0 owns the mutex! -- {}", i);
                        }
                        /*loop {
                            /*you can comment out this loop so that Task1 can successfully down the
                            counting_semaphore*/
                        }*/
                        match mutex0.semaphore_up() {
                            Ok(_) => {
                                trace!("Task0 dropped the mutex!");
                                kernel::task_end_scheduler();
                            }
                            Err(error) => {
                                trace!("mutex0 semaphore up triggers {}", error);
                            }
                        }
                    }
                    Err(error) => {
                        trace!("mutex0 semaphore take triggers {}", error);
                        task_timemanager::task_delay(pdMS_TO_TICKS!(1));
                        trace!("mutex0 delay in Err over!");
                    }
                }
            }
        };

        let task1 = move || {
            loop {
                match mutex1.semaphore_down(pdMS_TO_TICKS!(0)) {
                    Ok(_) => {
                        for i in 1..11 {
                            trace!("Task1 owns the mutex! -- {}", i);
                        }
                        task_timemanager::task_delay(pdMS_TO_TICKS!(1));
                        trace!("Task1's priority is {}", get_current_task_priority!());
                        /*loop {
                        }*/
                        match mutex1.semaphore_up() {
                            Ok(_) => {
                                trace!("Task1 dropped the mutex!");
                                task_timemanager::task_delay(pdMS_TO_TICKS!(1));
                                //     kernel::task_end_scheduler();
                            }
                            Err(error) => {
                                trace!("mutex1 semaphore up triggers {}", error);
                            }
                        }
                    }
                    Err(error) => {
                        trace!("mutex1 semaphore give triggers {}", error);
                    }
                }
            }
        };

        let Task0 = task_control::TCB::new()
            .name("Task0")
            .priority(4)
            .initialise(task0);

        let Task1 = task_control::TCB::new()
            .name("Task1")
            .priority(3)
            .initialise(task1);

        let Task12 = task_control::TCB::new()
            .name("Task2")
            .priority(3)
            .initialise(|| loop{});

        kernel::task_start_scheduler();
    }
    /**/ 
        #[test]
        fn test_counting_semaphore() {
            use simplelog::*;
            use semaphore::Semaphore;

            let _ = TermLogger::init(LevelFilter::Trace, Config::default());
            let cs0 = Arc::new(Semaphore::create_counting(2));
            let cs1 = Arc::clone(&cs0);
            let cs2 = Arc::clone(&cs0);

            let task_want_resources0 = move || {
                loop {
                    trace!("Enter Task0!");
                    match cs0.semaphore_down(pdMS_TO_TICKS!(10)) {
                        Ok(_) => {
                            for i in 1..11 {
                                trace!("cs0 owns the counting semaphore! -- {}", i);
                            }
                            // loop {
                                /*you can comment out this loop so that Task1 can successfully down the
                                counting_semaphore*/
                            // }
                            match cs0.semaphore_up() {
                                Ok(_) => {
                                    trace!("Task0 Finished!");
                                    break;
                                }
                                Err(error) => {
                                    trace!("cs0 semaphore give triggers {}", error);
                                }
                            }
                        },
                        Err(error) => {
                            trace!("cs0 semaphore take triggers {}", error);
                        },
                    }
                }
                loop {

                }
            };

            let task_want_resources1 = move || {
                loop {
                    trace!("Enter Task1!");
                    match cs1.semaphore_down(pdMS_TO_TICKS!(10)) {
                        Ok(_) => {
                            for i in 1..11 {
                                trace!("cs1 owns the counting semaphore! -- {}", i);
                            }
                            match cs1.semaphore_up() {
                                Ok(_) => {
                                    trace!("Test COUNTING SEMAPHORE COMPLETE!");
                                    kernel::task_end_scheduler();
                                    break;
                                }
                                Err(error) => {
                                    trace!("cs1 semaphore give triggers {}", error);
                                    kernel::task_end_scheduler();
                                }
                            }
                        },
                        Err(error) => {
                            trace!("cs1 semaphore take triggers {}", error);
                            kernel::task_end_scheduler();
                        },
                    }
                }
                loop {

                }
            };

            let task_want_resources2 = move || {
                loop {
                    trace!("Enter Task2!");
                    match cs2.semaphore_down(pdMS_TO_TICKS!(50)) {
                        Ok(_) => {
                            trace!("Task2 OK!");
                            for i in 1..11 {
                                trace!("cs2 owns the counting semaphore! -- {}", i);
                            }
                            loop {
                                /*you can comment out this loop so that Task1 can successfully down the
                                counting_semaphore*/
                            }
                            match cs2.semaphore_up() {
                                Ok(_) => {
                                    trace!("Task2 Finished!");
                                    break;
                                }
                                Err(error) => {
                                    trace!("cs2 semaphore give triggers {}", error);
                                }
                            }
                        },
                        Err(error) => {
                            trace!("cs2 semaphore take triggers {}", error);
                        },
                    }
                }
                loop {

                }
            };

            let _task0 = task_control::TCB::new()
                                    .name("Task0")
                                    .priority(3)
                                    .initialise(task_want_resources0);

            let _task1 = task_control::TCB::new()
                                    .name("Task1")
                                    .priority(3)
                                    .initialise(task_want_resources1);

            let _task2 = task_control::TCB::new()
                                    .name("Task2")
                                    .priority(3)
                                    .initialise(task_want_resources2);

            kernel::task_start_scheduler();

        }

        #[test]
        fn test_queue() {
            use std::fs::File;
            use simplelog::*;
            use queue_api::Queue;

            // 两个任务共享所有权，所以需Arc包装。
            let q_recv = Arc::new(Queue::new(10));
            let q_sender = Arc::clone(&q_recv);
            let _ = TermLogger::init(LevelFilter::Trace, Config::default());

            // 发送数据的任务代码。
            let sender = move || {
                for i in 1..11 {
                    // send方法的参数包括要发送的数据和ticks_to_wait
                    q_sender.send(i, pdMS_TO_TICKS!(50)).unwrap();
                }
                loop {

                }
            };

            // 接收数据的任务代码。
            let receiver = move || {
                let mut sum = 0;
                loop {
                    // receive方法的参数只有ticks_to_wait
                    if let Ok(x) = q_recv.receive(pdMS_TO_TICKS!(10)) {
                        println!("{}", x);
                        sum += x;
                    } else {
                        trace!("receive END");
                        // 若等待30ms仍未收到数据，则认为发送结束。
                        assert_eq!(sum, 55);
                        kernel::task_end_scheduler();
                    }
                }
            };

            // 创建这两个任务。
            let _sender_task = task_control::TCB::new()
                                .name("Sender")
                                .priority(3)
                                .initialise(sender);

            let _receiver_task = task_control::TCB::new()
                                .name("Receiver")
                                .priority(3)
                                .initialise(receiver);

            kernel::task_start_scheduler();
        }
}
