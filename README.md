# Request for Comments (RFC): WebAssembly Testing Standard (WAST)

**RFC Version:** 1.0  
**Status:** Draft  
**Authors:** Mattias Festin  
**Date:** 2025-01-03

## Abstract

This document defines the WebAssembly Testing Standard (WAST), a comprehensive standard designed to enable deterministic testing of WebAssembly (WASM) modules. The WAST standard introduces standardized WIT interfaces that mirror the WebAssembly System Interface (WASI) 0.2 specification, along with testing-specific features such as assertions, mocking capabilities, and test harnesses. Additionally, the WAST standard integrates DWARF debugging symbols to facilitate stack trace generation, enabling robust debugging and code coverage analysis.

The WAST standard is designed to be language-agnostic, requiring no modifications to the source code of the application being tested, thereby offering a standardized approach to deterministic testing for all languages that compile to WASM.

WAST is designed to seamlessly integrate with existing WebAssembly tooling and workflows. By adhering to the WASI specification and extending it with testing-specific features, WAST can be incorporated into current build and deployment pipelines without requiring significant changes. This allows developers to leverage their existing tools and processes while enhancing their testing capabilities with WAST's robust framework.

## 1. Motivation

WebAssembly provides a platform-independent, secure, and deterministic runtime environment. By extending its capabilities with a standardized testing framework, developers can unlock the following benefits:

- **Deterministic Testing:** Ensure consistent and reproducible test outcomes across environments.
  - **Finance:** Where accurate and consistent calculations are essential for trading algorithms, risk assessments, and financial reporting.
  - **Healthcare:** Where reliable software is crucial for patient data management, diagnostic tools, and medical devices.
  - **Embedded Systems:** Where predictable behavior is necessary for real-time systems, automotive software, and IoT devices.
- **Cross-Language Compatibility:** Provide a unified testing framework for all languages capable of compiling to WASM.
- **Enhanced Debugging:** Enable precise analysis of application behavior through structured stack traces and debugging tools.
- **Isolation and Mocking:** Separate testing logic from application logic, allowing for full mocking of I/O interactions via WASI.
- **Code Coverage and Error Analysis:** Facilitate advanced coverage analysis and error localization through DWARF integration.

By leveraging the inherent determinism of WebAssembly and the versatility of WASI, the WAST standard aims to establish a robust, cross-platform testing paradigm.

## 2. Specification

### 2.1 Architecture

The WAST standard operates using two WebAssembly modules:

- **Application Module:** This module encapsulates the core application logic, encompassing all primary business functionalities. It is responsible for executing the main computational tasks and performing input/output operations through the WebAssembly System Interface (WASI). The application module interacts with various WASI interfaces to handle file system access, network communication, and environment variable management, ensuring seamless integration with the host environment. By adhering to the WASI specification, the application module maintains platform independence and deterministic behavior, which are critical for consistent and reproducible testing outcomes.
- **Testing Module:**
    - **WASI Call Interception:** The testing module is responsible for intercepting all WASI calls originating from the application module. This interception mechanism ensures that the testing environment can fully control and simulate the responses to these calls, thereby enabling deterministic testing conditions.
    - **Test Case Management:** The module encapsulates a comprehensive suite of test cases, each designed to validate specific aspects of the application module's functionality. These test cases are meticulously organized and managed within the testing module, ensuring systematic and thorough coverage of the application's behavior.
    - **Assertions and Validation:** Built-in assertion mechanisms are provided to facilitate rigorous validation of application behavior. These assertions support a wide range of checks, including equality comparisons, range validations, and custom predicate evaluations, ensuring that the application meets its specified requirements under all tested conditions. The assertions API will also support asynchronous behavior testing, accommodating the event-driven nature of some WebAssembly use cases. This allows developers to validate asynchronous operations and ensure that the application behaves correctly in response to events and asynchronous tasks.
    - **Mocking Capabilities:** The testing module includes advanced mocking capabilities, allowing developers to define deterministic responses for various I/O operations such as file system access, network communication, and environment variable interactions. This feature enables the simulation of diverse scenarios and edge cases without altering the application code.
    - **Recursive Chaining and WASI Access:** The module optionally supports recursive chaining with other WAST modules, allowing for complex test scenarios that involve multiple layers of testing logic. This is particularly useful for multi-module WASM applications where inter-module dependencies need to be tested. By chaining testing modules, developers can simulate interactions between different parts of the application, ensuring that dependencies are correctly handled and that the overall system behaves as expected. Additionally, it can be configured to access real WASI interfaces when necessary, providing flexibility in testing both isolated and integrated environments.
    - **Additional Testing Logic:** Beyond basic test cases and assertions, the testing module can incorporate additional testing logic, such as setup and teardown routines, to prepare the environment for each test and ensure a clean state between test executions. This ensures that tests are isolated and do not interfere with each other, maintaining the integrity of the testing process.

By incorporating these features, the testing module within the WAST standard provides a robust and flexible environment for validating WebAssembly applications, ensuring that they perform reliably and correctly across diverse conditions and use cases.

### 2.2 WAST Testing Mode

The runtime must support a WAST Testing Mode, where the testing module is linked to the application module. In this mode:

- **WASI Call Interception:** All WASI calls from the application module are intercepted by the WAST testing WASM module.
- **Deterministic Execution:** The runtime ensures deterministic execution, isolating the application logic from external factors.
- **Compatibility:** WAST Testing Mode is designed to be compatible with existing WASM runtimes such as Wasmtime and Node.js WASM environments. This compatibility ensures that developers can integrate WAST into their current workflows without significant changes.
- **Resource Management:** The WAST standard includes mechanisms to manage resource constraints, especially in environments with limited memory or CPU power. This includes:
  - **Memory Management:** Efficient memory allocation and deallocation to prevent memory leaks and ensure that tests do not exceed available memory.
  - **CPU Usage:** Limiting CPU usage by controlling the execution time of tests and preventing infinite loops or excessive computation.
  - **Concurrency Control:** Managing concurrent test execution to avoid resource contention and ensure fair resource distribution among tests.

By incorporating these modifications, the WASM runtime can provide a robust and reliable environment for testing WebAssembly applications using the WAST standard.

### 2.3 WIT Interface Extensions

The WAST standard extends the WASI 0.2 specification with the following testing-specific WIT interfaces:

- **Setup/Teardown APIs:**
  - Facilitate the definition of setup and teardown routines to prepare the environment for each test and ensure a clean state between test executions.

- **Assertions:**
  - Include built-in methods for validating application behavior.
  - Support for equality checks, range validations, and custom predicates.

#### Example WIT Interface Definition for Setup/Teardown and Assertions

```wit
package wast:wast_snapshot_preview1@0.2.0;

interface asserts {
  /// Assert equality between two 32-bit integers
  assert_i32_eq: func(expected: i32, actual: i32) -> ();

  /// Assert that a 32-bit integer is within a specified range
  assert_i32_range: func(value: i32, min: i32, max: i32) -> ();

  /// Assert equality between two 64-bit integers
  assert_i64_eq: func(expected: i64, actual: i64) -> ();

  /// Assert that a 64-bit integer is within a specified range
  assert_i64_range: func(value: i64, min: i64, max: i64) -> ();

  /// Assert that two 32-bit floats are not within n steps of each other
  assert_f32_far: func(a: f32, b: f32, steps: u32) -> ();

  /// Assert that two 32-bit floats are within n steps of each other
  assert_f32_near: func(a: f32, b: f32, steps: u32) -> ();

  /// Assert that two 64-bit floats are not within n steps of each other
  assert_f64_far: func(a: f64, b: f64, steps: u32) -> ();

  /// Assert that two 64-bit floats are within n steps of each other
  assert_f64_near: func(a: f64, b: f64, steps: u32) -> ();
}
```

### 2.4 Stack Traces (Optional)

To enable advanced debugging and analysis, the WAST standard integrates with the DWARF debugging format. This allows developers to:

- Generate structured stack traces with source-level information.
- Precisely identify execution paths, bottlenecks, and errors.
- Integrate with existing debugging tools that support DWARF symbols.

While DWARF debugging symbols are not required for WAST, they are recommended as they provide better error messages and are necessary for code coverage analysis. The use of DWARF ensures compatibility with established debugging practices and tools.

This is an optional part of the WAST standard.

```
Authors comment: Maybe it is best to have the stack traces and debugging symbols part of a seperate standard?
```

## 3. Key Features

### 3.1 Deterministic Green Threads

WAST introduces deterministic green threads to simulate concurrency in a predictable manner. These lightweight threads are centrally planned within the testing framework so that:

- **Single Scheduler:** A single deterministic scheduler organizes thread execution, ensuring a consistent schedule for every run.  
- **Context Switching:** The scheduler context-switches between threads at predetermined or explicitly triggered points, preventing race conditions from impacting test outcomes.  
- **Thread Coordination:** Synchronization primitives (e.g., locks, semaphores) are also managed deterministically so that thread interactions remain reproducible.  

This way, developers can confidently write concurrent tests knowing that each run follows an identical execution order and timing, eliminating heisenbugs caused by non-deterministic thread schedules.

### 3.2 Garbage Collection

For garbage-collected languages, WAST provides mechanisms to simulate predictable garbage collection behaviors. This ensures that memory management does not introduce non-determinism into test results. The deterministic simulation of garbage collection aligns with the underlying WASM memory model by:

- **Controlled Memory Allocation:** Ensuring that memory allocation and deallocation follow a predictable pattern, consistent with the WASM linear memory model.
- **Reproducible GC Cycles:** Simulating garbage collection cycles at defined intervals or specific triggers, ensuring that the timing and impact of garbage collection are consistent across test runs.
- **Memory Usage Tracking:** Monitoring memory usage to ensure that garbage collection occurs under controlled conditions, preventing unexpected behavior due to memory pressure.

### 3.3 Code Coverage

WAST facilitates comprehensive code coverage analysis through DWARF-based stack traces. The framework will support both branch and statement coverage metrics, providing detailed insights into the execution paths and code coverage of the application. These metrics will be visualized using:

- **Coverage Reports:** Generating detailed reports that highlight covered and uncovered branches and statements, allowing developers to identify gaps in test coverage.
- **Visual Tools:** Integrating with existing code coverage visualization tools to provide graphical representations of coverage data, making it easier to understand and analyze coverage metrics.
- **Interactive Dashboards:** Offering interactive dashboards that allow developers to explore coverage data, filter by specific criteria, and drill down into detailed coverage information.

### 3.4 DWARF Integration

WAST adopts DWARF as the standard for debugging symbols, enabling source-level mapping and facilitating seamless integration with existing tools. Developers can use tools like wasm-sourcemap to generate DWARF-compliant mappings during the compilation process.

## 4. Example Workflow

### 4.1 Configuration

- Compile the application into a WASM module with DWARF debugging symbols.
- Create a testing module with WIT bindings for mocks and assertions.

### 4.2 Execution

- Run the test suite in WAST Testing Mode, linking the testing module to the application module. Mocked WASI calls are intercepted and validated against the test definitions.

### 4.3 Analysis

- Inspect the results of assertions and stack traces to identify issues. Use DWARF symbols to trace errors back to the source code.

## 5. Benefits

- **Reproducibility:** Guarantees consistent test results across identical inputs and environments.
- **Language-Agnostic:** Enables testing for any language capable of compiling to WASM.
- **Enhanced Debugging:** Provides robust debugging tools and structured error analysis.
- **Isolation:** Testing logic is encapsulated, ensuring no dependencies on external systems.
- **Code Coverage:** Facilitates comprehensive coverage analysis through DWARF-based stack traces.

## 6. Future Considerations

- **WASI Extensions:** Expand support for future WASI versions and new I/O capabilities.
- **Advanced Coverage Tools:** Develop utilities for visualizing and analyzing code coverage.
- **Standardized Test Reporting:** Define a format for test results to enable interoperability between testing tools.

## 7. Backwards Compatibility and Changes

The WebAssembly Testing Standard (WAST) has been meticulously designed to ensure seamless integration with existing WebAssembly (WASM) modules, thereby facilitating their testing without necessitating any modifications. This backward compatibility is a cornerstone of the WAST standard, ensuring that the adoption of this standard does not disrupt current development workflows or require extensive refactoring of existing codebases.

### 7.1 Compatibility with Existing WASM Modules

WAST is a superset of the existing WASM and WebAssembly System Interface (WASI) standards. This means that any WASM module adhering to the current WASI specification can be tested using the WAST standard without any changes. The WAST standard extends the capabilities of WASI by introducing additional interfaces and functionalities specifically tailored for testing, while maintaining full compatibility with the core WASI APIs.

### 7.2 WASI API Export Modifications

One of the significant enhancements introduced by WAST is the modification of how WASI API functions are handled. In the traditional WASI model, API functions are imported into the WASM module. However, under the WAST standard, these functions are exported from the WASM module. This inversion allows the testing module to intercept and mock these calls, thereby providing a controlled and deterministic testing environment.

### 7.3 Runtime Support for WAST Testing Mode

To fully leverage the capabilities of the WAST standard, the WASM runtime must support a specialized "WAST Testing Mode." In this mode, the runtime is responsible for linking the testing module with the application module and ensuring that all WASI calls from the application module are intercepted by the testing module. This interception is crucial for maintaining deterministic execution and isolating the application logic from external factors that could introduce variability into the test results.

The implementation of WAST Testing Mode requires the following modifications to the WASM runtime:

- **Interception Mechanism:** The runtime must implement a mechanism to intercept all WASI calls made by the application module and redirect them to the testing module.
- **Deterministic Execution:** The runtime must ensure that the execution of the application module is deterministic, meaning that given the same inputs, the outputs will be consistent across different runs and environments.
- **Isolation:** The runtime must isolate the application logic from external influences, such as file system state or network conditions, to ensure that tests are reproducible and not affected by external variability.

By incorporating these modifications, the WASM runtime can provide a robust and reliable environment for testing WebAssembly applications using the WAST standard.

## 7.4 WAST Versioning Alignment with WASI

WAST versions will match the corresponding WASI release versions to provide a clear indication of which WASI APIs are supported. For example, if WAST is labeled as `wast_snapshot_preview1@0.2.0`, it corresponds to `wasi_snapshot_preview1@0.2.0`. This ensures that developers can easily track the available WASI APIs and maintain compatibility between the application and testing modules.

## 7.5 Deterministic Handling of Additional WASI Modules

Beyond core WASI APIs, WAST also supports deterministic handling of specialized WASI modules—like clocks, random, filesystem, sockets, CLI, HTTP, and others. For example:

- **Random:** A fixed seed can be specified so that calls to random functions always produce the same outputs.  
- **Clock:** An arbitrary start time can be set, and each call can advance this “virtual” clock predictably (e.g., incrementing by a constant offset).  
- **Filesystem & Network:** Interactions can use “middleware” hooks in the WAST WASM module or the compiled test framework. This allows snapshot-based tests to remain consistent while selectively varying data (e.g., modifying authentication tokens).  
- **And more ...**

These mechanisms are not part of the core WAST standard but can be implemented in the custom test framework compiled into the WAST WASM module. They ensure reproducible I/O behavior while allowing scenario-specific modifications for each test run.

## 8. Conclusion

The WebAssembly Testing Standard (WAST) represents a significant advancement in the testing of WebAssembly applications. By building upon the existing WASM and WASI standards, WAST introduces a comprehensive framework that enables deterministic testing, cross-language compatibility, and enhanced debugging capabilities. The backward compatibility of WAST ensures that existing WASM modules can be tested without any modifications, while the introduction of WAST Testing Mode in the runtime provides the necessary infrastructure for controlled and reproducible testing environments.

The adoption of WAST by the WebAssembly community and its potential standardization by the W3C will pave the way for more robust and reliable WebAssembly applications. By providing a unified testing paradigm, WAST will facilitate the development of high-quality software that meets the rigorous demands of modern computing environments.

In conclusion, the WebAssembly Testing Standard (WAST) is poised to become an essential tool for developers, offering a standardized approach to testing that enhances the reliability, maintainability, and overall quality of WebAssembly applications. The formalization of this standard will mark a significant milestone in the evolution of WebAssembly, reinforcing its position as a leading technology for secure, efficient, and cross-platform software development.

The WebAssembly Testing Standard (WAST) provides a robust, deterministic framework for testing WebAssembly applications. By leveraging WASI, DWARF, and a standardized testing architecture, WAST ensures reproducibility, cross-language compatibility, and enhanced debugging capabilities. This standard represents a significant step forward in creating a unified testing paradigm for the WebAssembly ecosystem.

## 9. Standardization Scope

The WebAssembly Testing Standard (WAST) defines a standard for testing WebAssembly applications. However, it is important to clarify the boundary of WAST within the broader WebAssembly ecosystem:

- **Optional Testing Standard:** WAST is designed as an optional testing standard for WebAssembly applications. While it provides robust and deterministic testing capabilities, it is not a mandatory part of WebAssembly compliance. Developers and organizations can choose to adopt WAST to enhance their testing processes, but it is not required for a WebAssembly runtime to implement to be WASM compliant.
- **Integration with Existing Standards:** WAST extends the WebAssembly System Interface (WASI) specification with testing-specific features, ensuring compatibility with existing WebAssembly standards. It leverages DWARF debugging symbols for enhanced debugging and code coverage analysis, but these features are optional and recommended for improved testing outcomes.
- **Flexibility and Extensibility:** The WAST framework is designed to be flexible and extensible, allowing it to adapt to evolving WebAssembly standards and incorporate new features as they become available. This ensures that WAST can remain relevant and useful as the WebAssembly ecosystem grows and evolves.

By defining WAST as an optional testing standard, the WebAssembly community can benefit from its advanced testing capabilities without imposing additional requirements on developers and organizations. This approach encourages the adoption of WAST while maintaining the flexibility and simplicity of the core WebAssembly standards.

## 10. Interoperability

As the WebAssembly ecosystem evolves, multiple testing efforts and frameworks may emerge. It is important to address potential conflicts or overlaps with other WebAssembly testing efforts or frameworks and propose ways to resolve them:

- **Conflict Resolution:** In cases where WAST conflicts with other WebAssembly testing frameworks, a collaborative approach should be taken to harmonize the standards. This may involve joint discussions and working groups to align the testing methodologies and ensure compatibility.
- **Standardization Efforts:** Engage with the WebAssembly community and relevant standardization bodies to ensure that WAST complements existing efforts rather than duplicating them. This can be achieved by participating in community discussions, contributing to WebAssembly proposals, and aligning WAST with broader WebAssembly goals.
- **Modular Design:** The modular design of WAST allows for the integration of other testing frameworks and tools. By providing clear interfaces and extension points, WAST can coexist with other frameworks, allowing developers to choose the best tools for their specific needs.
- **Documentation and Guidelines:** Provide comprehensive documentation and guidelines on how to integrate WAST with other testing frameworks. This includes best practices for combining WAST with other tools, handling potential conflicts, and ensuring a seamless testing experience.
- **Community Feedback:** Actively seek feedback from the WebAssembly community to identify potential overlaps and areas for improvement. This feedback can be used to refine WAST and ensure that it meets the needs of developers while maintaining compatibility with other frameworks.

By addressing interoperability and potential conflicts proactively, WAST can coexist with other WebAssembly testing efforts, providing a unified and flexible testing ecosystem for WebAssembly applications.

## Glossary

- **DWARF**: A debugging file format used for storing and describing debugging information.
- **WASI**: WebAssembly System Interface, a modular system interface for WebAssembly.
- **WIT**: WebAssembly Interface Types, a proposal for defining and using complex data types in WebAssembly.
- **WAST**: WebAssembly Testing Standard, a framework for testing WebAssembly applications.
- **Deterministic Green Threads**: Lightweight threads managed within the testing framework to simulate concurrency in a predictable manner, ensuring consistency across test runs.
- **Snapshot Testing**: A testing technique where the output of a function or UI component is captured and compared against a stored snapshot to detect changes.
- **Test Harness**: A collection of software and test data configured to test a program unit by running it under varying conditions and monitoring its behavior and outputs.

## Examples

### Example 1: Basic Test Case
__WASM app module__
```wasm
(module
  (import "wasi_snapshot_preview1" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))
  (func (export "main")
    ;; Application logic here that does some file stuff
  )
)
```

__WAST module__
```wasm
(module
  ;; Note the namespace of the WASM module to be tested is "wast_application"
  (import "wast_application" "main" (func $main))
  (import "wast_snapshot_preview1" "assert_i32_eq" (func $assert_i32_eq (param i32 i32)))

  ;; Setup and teardown
  (func (export "wst_before_all")
     ;;...
  )
  (func (export "wst_before_each")
    ;;...
  )
  (func (export "wst_after_all")
    ;;...
  )
  (func (export "wst_after_each")
    ;;...
  )

  ;; Tests (Each test is run in a separate context to keep them isolated an deterministic for concurrent testing).
  ;; Note that each test is prefixed with "wst_test_" to indicate that they are test functions.
  (func (export "wst_test_example1")
    ;; Test logic for example1
  )
  (func (export "wst_test_example2")
    ;; Test logic for example2
  )

  ;; WASI mocks
  (func $fd_write (param i32 i32 i32 i32) (result i32)
    ;; Mock implementation of fd_write here...
    ;; ...
  )
  (export "fd_write" (func $fd_write))
)