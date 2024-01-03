# Nodex

### Problem Statement: ROS and FPrime… suck. 
- ROS: heavy/bloated, complex to learn and use
- FPrime: C++ only, undocumented, XML/CMake building and debugging hell

### Solution: 
- Make a new framework for modular, component-based codebases, like those used in robotics or aerospace.
- Priority should be ease of unit testing and running with input spoofing. 
- Should work with multiple languages and generate *USEFUL* errors. 
  - Since multiple languages within one project are supported, each node can leverage the libraries written for each language. This is a key feature of Nodex -- giving developers to use the languages they are most comfortable with or that are best suited for the task at hand.

General important steps to a functioning proof of concept:
1. Determine a messaging protocol (defining a schema for the payload of a message on a connection) [timestamp, connection, sending and receiving components, etc.]
2. Determine a multi-process communication middleware (ZeroMQ or gRPC)
3. Determine component and connection instantiation schema (JSON)
4. Create APIs for each language in question (Python, C++, Rust, JS, etc.), with the main CLI written in Rust
4. Implement error handling/propogation
6. Component lifecycle manager (start/stop/monitor components)
7. Runtime config on a component and system level that affects runtime but does NOT require a re-compile of compiled components
8. Robust tests of the framework internal code + create a method for testing components
