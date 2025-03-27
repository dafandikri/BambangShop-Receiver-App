# BambangShop Receiver App

Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project

In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:

1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a Rocket web framework skeleton that you can work with.

As this is an Observer Design Pattern tutorial repository, you need to implement a feature: `Notification`.
This feature will receive notifications of creation, promotion, and deletion of a product, when this receiver instance is subscribed to a certain product type.
The notification will be sent using HTTP POST request, so you need to make the receiver endpoint in this project.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Receiver" folder.

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment

1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    ROCKET_PORT=8001
    APP_INSTANCE_ROOT_URL=http://localhost:${ROCKET_PORT}
    APP_PUBLISHER_ROOT_URL=http://localhost:8000
    APP_INSTANCE_NAME=Safira Sudrajat
    ```
    Here are the details of each environment variable:
    | variable | type | description |
    |-------------------------|--------|-----------------------------------------------------------------|
    | ROCKET_PORT | string | Port number that will be listened by this receiver instance. |
    | APP_INSTANCE_ROOT_URL | string | URL address where this receiver instance can be accessed. |
    | APP_PUUBLISHER_ROOT_URL | string | URL address where the publisher instance can be accessed. |
    | APP_INSTANCE_NAME | string | Name of this receiver instance, will be shown on notifications. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)
3.  To simulate multiple instances of BambangShop Receiver (as the tutorial mandates you to do so),
    you can open new terminal, then edit `ROCKET_PORT` in `.env` file, then execute another `cargo run`.

    For example, if you want to run 3 (three) instances of BambangShop Receiver at port `8001`, `8002`, and `8003`, you can do these steps:

    - Edit `ROCKET_PORT` in `.env` to `8001`, then execute `cargo run`.
    - Open new terminal, edit `ROCKET_PORT` in `.env` to `8002`, then execute `cargo run`.
    - Open another new terminal, edit `ROCKET_PORT` in `.env` to `8003`, then execute `cargo run`.

## Mandatory Checklists (Subscriber)

- [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop-receiver to a new repository.
- **STAGE 1: Implement models and repositories**
  - [ ] Commit: `Create Notification model struct.`
  - [ ] Commit: `Create SubscriberRequest model struct.`
  - [ ] Commit: `Create Notification database and Notification repository struct skeleton.`
  - [ ] Commit: `Implement add function in Notification repository.`
  - [ ] Commit: `Implement list_all_as_string function in Notification repository.`
  - [ ] Write answers of your learning module's "Reflection Subscriber-1" questions in this README.
- **STAGE 3: Implement services and controllers**
  - [ ] Commit: `Create Notification service struct skeleton.`
  - [ ] Commit: `Implement subscribe function in Notification service.`
  - [ ] Commit: `Implement subscribe function in Notification controller.`
  - [ ] Commit: `Implement unsubscribe function in Notification service.`
  - [ ] Commit: `Implement unsubscribe function in Notification controller.`
  - [ ] Commit: `Implement receive_notification function in Notification service.`
  - [ ] Commit: `Implement receive function in Notification controller.`
  - [ ] Commit: `Implement list_messages function in Notification service.`
  - [ ] Commit: `Implement list function in Notification controller.`
  - [ ] Write answers of your learning module's "Reflection Subscriber-2" questions in this README.

## Your Reflections

This is the place for you to write reflections:

### Mandatory (Subscriber) Reflections

#### Reflection Subscriber-1

1. **Use of RwLock vs. Mutex for Notification Synchronization:**

   In the BambangShop Receiver implementation, we use `RwLock<Vec<Notification>>` to synchronize access to our notifications list. This choice is deliberate and offers significant benefits for our use case:

   `RwLock` (Read-Write Lock) allows multiple readers to access the data simultaneously OR a single writer to have exclusive access. This is particularly well-suited for our notification system because:

   - The list of notifications is frequently read (when users request to view notifications) but less frequently written to (only when new notifications arrive)
   - Multiple concurrent readers can access the notification list without blocking each other
   - The performance penalty only occurs when writing new notifications, which happens less often

   If we had used `Mutex` instead, every access to the notification list - whether for reading or writing - would be exclusive. This would create unnecessary contention when multiple clients simultaneously request to view notifications, as they would need to wait for each other despite performing non-conflicting read operations.

   The difference becomes especially significant as the system scales with more concurrent users. With `RwLock`, read operations can proceed in parallel, greatly improving throughput for notification display operations.

2. **Rust's Static Variables vs. Java's Approach:**

   In our implementation, we use the `lazy_static!` macro to define `NOTIFICATIONS` as a "static" variable. This approach differs significantly from Java's handling of static variables:

   In Java, you can directly mutate static variables through static methods without special handling. However, Rust intentionally restricts mutation of static variables due to its commitment to memory safety and preventing data races.

   Rust enforces these restrictions because:

   - Static variables in Rust have a 'static lifetime, meaning they exist for the entire duration of the program
   - Multiple threads can access static variables concurrently, potentially leading to data races
   - Rust's ownership system, which normally prevents data races, doesn't apply to static variables in the same way

   While Rust does provide `static mut` variables, accessing them requires using `unsafe` blocks, as the compiler cannot guarantee thread safety. The `lazy_static!` macro offers a safe alternative by:

   - Initializing the static variable only when first accessed (lazy initialization)
   - Wrapping the content in synchronization primitives like `RwLock` or `Mutex`
   - Providing interior mutability that maintains Rust's safety guarantees

   This approach maintains Rust's promise of thread safety while still allowing us to have globally accessible, mutable state when needed.

#### Reflection Subscriber-2

1. **Exploring Code Outside the Tutorial Steps:**

   Yes, I explored additional code outside the tutorial steps, particularly `src/lib.rs`, which provided valuable insights into the application structure. This file contains the core bootstrapping logic and configuration management through the `AppConfig` struct, which handles environment variables and app-wide constants like `REQWEST_CLIENT`.

   Examining this code helped me understand how the application's configuration is centralized and loaded from the `.env` file, and how the HTTP client is instantiated as a singleton. I also discovered the custom error handling system that's used throughout the application, which wraps errors in a consistent format for API responses.

   This exploration was particularly useful when debugging connection issues between the Publisher and Receiver, as it revealed how URLs are constructed and how the application identity is maintained across different instances.

2. **Observer Pattern and Scalability:**

   The Observer pattern significantly simplifies adding new subscribers to the system. When spawning multiple instances of the Receiver app on different ports, each instance can independently subscribe to product types of interest without any changes needed in the Publisher app. This demonstrates the pattern's strength in providing loose coupling between publishers and subscribers.

   The Publisher doesn't need to know anything about the implementation details of its subscribers - it only needs to maintain a list of subscriber endpoints and notify them when events occur. This decoupling allows the system to scale horizontally with minimal coordination overhead.

   However, scaling the Publisher side would be more challenging. If we wanted to run multiple Publisher instances, we would need additional infrastructure like a shared database or message queue to ensure that subscriber registrations are propagated across all Publisher instances. The current implementation stores subscribers in memory using `lazy_static`, which doesn't automatically synchronize across multiple application instances.

   To support multiple Publishers, we would need to implement a more sophisticated subscription management system, possibly using a distributed data store or service discovery mechanism.

3. **Testing and Documentation:**

   I enhanced the Postman collection with pre-request scripts to automatically set environment variables, which simplified testing across different subscriber instances. The tests verify successful notification reception and proper display formatting.

   These testing enhancements proved extremely valuable for:

   - Validating the correct handling of different notification types (creation, promotion, deletion)
   - Ensuring that notifications are properly formatted in the receiver's display
   - Testing edge cases like unsubscribing and re-subscribing to the same product type
   - Verifying that multiple receiver instances can coexist without interference

   Additionally, I documented expected response formats and potential error states in the Postman collection, which served as living documentation for the API. This became particularly useful when troubleshooting integration issues between the Publisher and Receiver applications.

   For my group project, I plan to incorporate these testing practices early in development to catch integration issues before they propagate throughout the system.
