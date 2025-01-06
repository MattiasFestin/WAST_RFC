# Request for Comments (RFC): WebAssembly Testing (WAST) standard

**RFC Version:** 1.0  
**Status:** Draft  
**Authors:** Mattias Festin  
**Date:** 2025-01-03

## Abstract

This document defines the WebASsembly Testing (WAST) standard, a comprehensive standard designed to enable deterministic testing of WebAssembly (WASM) modules. The WAST standard introduces testing-specific features such as assertions (implemented via language-level panics), mocking capabilities, and test harnesses. Additionally, the WAST standard integrates DWARF debugging symbols to facilitate stack trace generation, enabling robust debugging and code coverage analysis.

The WAST standard is designed to be language-agnostic, requiring no modifications to the source code of the application being tested, thereby offering a standardized approach to deterministic testing for all languages that compile to WASM.

WAST is designed to seamlessly integrate with existing WebAssembly tooling and workflows. By adhering to the WASI specification and extending it with testing-specific features, WAST can be incorporated into current build and deployment pipelines without requiring significant changes. This allows developers to leverage their existing tools and processes while enhancing their testing capabilities with WAST.

## 1. Motivation

WebAssembly provides a platform-independent, secure, and deterministic runtime environment. By extending its capabilities with a standardized testing framework, developers can unlock the following benefits:

- **Deterministic Testing:** Ensure consistent and reproducible test outcomes across environments.
  - **Finance:** Where accurate and consistent calculations are essential for trading algorithms, risk assessments, and financial reporting.
  - **Healthcare:** Where reliable software is crucial for patient data management, diagnostic tools, and medical devices.
  - **Embedded Systems:** Where predictable behavior is necessary for real-time systems, automotive software, and IoT devices.
- **Cross-Language Compatibility:** Provide a unified testing standard for all languages capable of compiling to WASM. A solution for this will be needed for mixed lanugage senarios with multiple WASM modules.
- **Isolation and Mocking:** Separate testing logic from application logic, allowing for full mocking of I/O interactions via WASI.
- **Code Coverage and Error Analysis:** Facilitate advanced coverage analysis and error localization through DWARF integration.

By leveraging the inherent determinism of WebAssembly and the versatility of WASI, the WAST standard aims to establish a robust, cross-platform testing paradigm.

## 2. Specification

### 2.1 Architecture

The WAST standard operates using two WebAssembly modules:

- **Application Module:**  
  This module encapsulates the core application logic, encompassing all primary business functionalities. It is responsible for executing the main computational tasks and performing input/output operations through the WebAssembly System Interface (WASI). The application module interacts with various WASI interfaces to handle file system access, network communication, and environment variable management, ensuring seamless integration with the host environment. By adhering to the WASI specification, the application module maintains platform independence and deterministic behavior, which are critical for consistent and reproducible testing outcomes.

- **Testing Module:**  
  - **WASI Call Interception:** The testing module is responsible for intercepting all WASI calls originating from the application module. This interception mechanism ensures that the testing environment can fully control and simulate the responses to these calls, thereby enabling deterministic testing conditions.  
  - **Test Case Management:** The module contains a comprehensive suite of test cases, each designed to validate specific aspects of the application module's functionality. These test cases can each panic on failure, ensuring that failing tests halt execution in a clear and consistent manner.  
  - **Mocking Capabilities:** The testing module includes advanced mocking capabilities, allowing developers to define deterministic responses for various I/O operations such as file system access, network communication, and environment variable interactions. This feature enables the simulation of diverse scenarios and edge cases without altering the application code.
  - **Recursive Chaining and WASI Access:** The module optionally supports recursive chaining with other WAST modules, allowing for complex test scenarios that involve multiple layers of testing logic. It can be configured to access real WASI interfaces when necessary, providing flexibility in testing both isolated and integrated environments.
  - **Additional Testing Logic:** Beyond basic test cases, the testing module may include any additional test routines or utilities needed by the developer’s setup. Each test remains fully isolated, ensuring reliable and reproducible results.

### 2.2 WAST Testing Mode

The runtime must support a WAST Testing Mode, where the testing module is linked to the application module. In this mode:

- **WASI Call Interception:** All WASI calls from the application module are intercepted by the WAST testing WASM module.  
- **Deterministic Execution:** The runtime ensures deterministic execution, isolating the application logic from external factors.  
- **Compatibility:** WAST Testing Mode is designed to be compatible with existing WASM runtimes such as Wasmtime and Node.js WASM environments. This compatibility ensures that developers can integrate WAST into their current workflows without significant changes.  
- **Resource Management:** The WAST standard includes mechanisms to manage resource constraints, especially in environments with limited memory or CPU power.

### 2.3 Stack Traces

To enable advanced debugging and analysis, the WAST standard integrates with the DWARF debugging format. This allows developers to:

- Generate structured stack traces with source-level information.  
- Precisely identify execution paths, bottlenecks, and errors.  
- Integrate with existing debugging tools that support DWARF symbols.

While DWARF debugging symbols are not required for WAST, they are recommended as they provide better error messages and are necessary for code coverage analysis. The use of DWARF ensures compatibility with established debugging practices and tools.

```
Authors comment: Maybe it is best to have the stack traces and debugging symbols part of a seperate standard?
```

## 3. Key Features

### 3.1 Deterministic Green Threads

WAST introduces deterministic green threads to simulate concurrency in a predictable manner. These lightweight threads are centrally planned from the WASM runtime so that:

- **Single Scheduler:** A single deterministic scheduler organizes thread execution, ensuring a consistent schedule for every run.  
- **Context Switching:** The scheduler context-switches between threads at predetermined or explicitly triggered points, preventing race conditions from impacting test outcomes.  
- **Thread Coordination:** Synchronization primitives (e.g., locks, semaphores) are also managed deterministically so that thread interactions remain reproducible.  

This way, developers can confidently write concurrent tests knowing that each run follows an identical execution order and timing, eliminating heisenbugs caused by non-deterministic thread schedules.

#### Optional Non-Deterministic Scheduling mode

In addition to a strictly deterministic scheduler, WAST may offer an optional mode that randomly interleaves thread execution. This helps expose race conditions and concurrency bugs by simulating out-of-order thread scheduling in a controlled testing environment.

### 3.2 Garbage Collection

For garbage-collected languages, WAST provides mechanisms to simulate predictable garbage collection behaviors. This ensures that memory management does not introduce non-determinism into test results. The deterministic simulation of garbage collection aligns with the underlying WASM memory model by:

- **Controlled Memory Allocation:** Ensuring that memory allocation and deallocation follow a predictable pattern, consistent with the WASM linear memory model.
- **Reproducible GC Cycles:** Simulating garbage collection cycles at defined intervals or specific triggers, ensuring that the timing and impact of garbage collection are consistent across test runs.
- **Memory Usage Tracking:** Monitoring memory usage to ensure that garbage collection occurs under controlled conditions, preventing unexpected behavior due to memory pressure.

#### Optional Non-Deterministic GC Mode

In addition to predictable GC cycles, WAST can optionally introduce non-deterministic GC pauses. This mode allows the testing runtime to trigger garbage collection at arbitrary intervals, reflecting real-world scenarios where GC timing may vary. Combining this mode with non-deterministic thread scheduling helps uncover concurrency bugs that hinge on specific GC timing.

### 3.3 Code Coverage

WAST facilitates comprehensive code coverage analysis through DWARF-based stack traces. The standard supports both branch and statement coverage metrics, providing detailed insights into the execution paths and code coverage of the application. These metrics will be visualized using:

- **Coverage Reports:** Generating detailed reports that highlight covered and uncovered branches and statements, allowing developers to identify gaps in test coverage.
- **Visual Tools:** Integrating with existing code coverage visualization tools to provide graphical representations of coverage data, making it easier to understand and analyze coverage metrics.
- **Interactive Dashboards:** Offering interactive dashboards that allow developers to explore coverage data, filter by specific criteria, and drill down into detailed coverage information.

### 3.4 DWARF Integration

WAST adopts DWARF as the standard for debugging symbols, enabling source-level mapping and facilitating seamless integration with existing tools. Developers can use tools like wasm-sourcemap to generate DWARF-compliant mappings during the compilation process.

### 3.5 Property-Based Testing and Fuzzing

WAST supports advanced testing methodologies such as Property-Based Testing (PBT) and fuzzing, leveraging its deterministic execution and I/O control capabilities.

#### Property-Based Testing (PBT)

Property-Based Testing involves defining properties that the application should always satisfy and then generating a wide range of inputs to test these properties. WAST facilitates PBT by:

- **Deterministic Execution:** Ensuring that each test run is reproducible, allowing for consistent validation of properties across different environments.
- **Mocking Capabilities:** Allowing developers to mock I/O interactions, enabling the simulation of diverse scenarios and edge cases without altering the application code.
- **Cross-Language Compatibility:** Providing a unified testing standard for all languages capable of compiling to WASM, making it easier to implement PBT across different projects.

#### Fuzzing

Fuzzing involves providing random or semi-random inputs to the application to discover unexpected behaviors or vulnerabilities. WAST enhances fuzzing by:

- **Controlled Randomness:** Allowing the use of fixed seeds to ensure that fuzzing runs are reproducible, making it easier to debug and analyze issues.
- **I/O Mocking:** Enabling the simulation of various edge cases and environments by mocking I/O interactions, helping to uncover hidden bugs and vulnerabilities.
- **Integration with Existing Tools:** Seamlessly integrating with existing fuzzing tools and workflows, allowing developers to leverage their current toolsets while enhancing their testing capabilities with WAST.

By leveraging WAST's deterministic execution, mocking capabilities, and integration with existing tools, developers can perform comprehensive PBT and fuzzing, ensuring robust and reliable WebAssembly applications.

### 3.6 Mutation Testing

Mutation testing is a technique used to evaluate the quality of test suites by introducing small changes (mutations) to the application's code and checking if the tests can detect these changes. WAST's deterministic nature makes it an excellent fit for mutation testing, providing a controlled environment to identify potential weaknesses in the test suite and uncover hidden bugs in the application.

#### Benefits of Mutation Testing with WAST

- **Deterministic Execution:** Ensures that each mutation test run is reproducible, allowing for consistent evaluation of test suite effectiveness.
- **Controlled Mutations:** Allows for precise control over the mutations introduced, enabling targeted testing of specific parts of the application.
- **Isolation:** Each mutation is tested in isolation, ensuring that the results are not influenced by other factors.
- **Enhanced Debugging:** Integration with DWARF debugging symbols helps in tracing the impact of mutations back to the source code, facilitating easier debugging and analysis.

#### How Mutation Testing Works with WAST

1. **Introduce Mutations:** Small changes are introduced to the application's WebAssembly module. These mutations can include altering arithmetic operations, changing control flow, or modifying function calls.
2. **Run Tests:** The test suite is executed in WAST Testing Mode with the mutated application module.
3. **Analyze Results:** The results are analyzed to determine if the tests detected the mutations. If a mutation is not detected, it indicates a potential weakness in the test suite.
4. **Iterate:** The process is repeated with different mutations to comprehensively evaluate the test suite.

#### Mutation Testing vs. Fuzzing

While both mutation testing and fuzzing are techniques used to uncover bugs, they operate differently and are complementary:

- **Fuzzing:** This technique involves feeding pseudo-random inputs to a program to find bugs, particularly useful for programs that parse complex or untrusted inputs such as binary file formats or network protocols. Fuzzing tends to find bugs triggered by complex or unusual inputs.
- **Mutation Testing:** This technique makes algorithmically-generated changes to a copy of the program source and measures whether the test suite catches the change. Mutation testing tends to highlight logic that might be correct but is not adequately tested.

By leveraging WAST's deterministic execution and advanced debugging capabilities, mutation testing can be effectively used to enhance the robustness and reliability of WebAssembly applications.

### 3.7 Performance Testing

WAST supports performance testing to ensure that WebAssembly applications meet specified performance criteria. The performance test function in the WAST module provides detailed metrics from the last run and the current run, including execution time and resource usage. This allows end users to define and enforce their own performance policies.

#### Performance Metrics Provided

- **Execution Time:** The amount of time taken for the last run and the current run.
- **Resource Usage:** The resources used during the last run and the current run, such as CPU and memory.

#### How Performance Testing Works with WAST

1. **Run Performance Tests:** Execute the performance tests in WAST Testing Mode. The WAST module will collect and provide metrics for execution time and resource usage for both the last run and the current run.
2. **Analyze Results:** Compare the metrics from the last run and the current run to determine if the application meets the specified performance criteria. Users can define their own policies based on these metrics.
3. **Iterate:** Adjust the application or performance criteria as needed and repeat the testing process to ensure that performance remains within acceptable limits.

By leveraging WAST's deterministic execution and advanced monitoring capabilities, performance testing can be effectively used to ensure that WebAssembly applications meet specified performance criteria and maintain acceptable performance levels.

### 3.8 Shadow Testing

Shadow testing, also known as parallel testing, involves running a new version of an application alongside the old version to compare their outputs and ensure that the new version behaves as expected. This technique is particularly useful when replacing or upgrading an existing application, as it allows for full application simulation and comparison.

#### Benefits of Shadow Testing with WAST

- **Validation:** Ensures that the new application version produces the same results as the old version, validating correctness and consistency.
- **Regression Detection:** Identifies any regressions or discrepancies between the old and new versions, allowing for early detection and resolution of issues.
- **Seamless Transition:** Facilitates a smooth transition from the old application to the new one by providing confidence that the new version performs as expected.

#### How Shadow Testing Works with WAST

1. **Run Both Versions:** Execute both the old and new versions of the application in parallel using WAST Testing Mode.
2. **Compare Outputs:** Capture and compare the outputs of both versions to identify any differences. This can include comparing return values, I/O interactions, and performance metrics.
3. **Analyze Discrepancies:** Analyze any discrepancies between the outputs to determine their cause and address any issues in the new version.
4. **Iterate:** Make necessary adjustments to the new application and repeat the shadow testing process until the outputs match and the new version is validated.

By leveraging WAST's deterministic execution and advanced monitoring capabilities, shadow testing can be effectively used to ensure that a new application version behaves consistently with the old version, facilitating a smooth and reliable transition.

## 4. Example Workflow

### 4.1 Configuration

- Compile the application into a WASM module with DWARF debugging symbols.

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

WAST versions will match the corresponding WASI release versions to provide a clear indication of which WASI APIs are supported. For example, if WASI verion is `0.2.0`, it corresponds to the WAST version `0.2.0`. This ensures that developers can easily track the available WASI APIs and maintain compatibility between the application and testing modules.

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
- **Integration with Existing Standards:** WAST extends the WebAssembly System Interface (WASI) specification with testing-specific features, ensuring compatibility with existing WebAssembly standards. It leverages DWARF debugging symbols for enhanced debugging and code coverage analysis.
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
- **WAST**: WebASsembly Testing, a standard for testing WebAssembly applications.
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
  ;; Note the namespace of the WASM module to be tested is "wast_application" so all global exported functions will be under this namespace
  (import "wast_application" "main" (func $main))

  ;; Tests (Each test is run in a separate context to keep them isolated an deterministic for concurrent testing).
  ;; Note that each test is prefixed with "wast_test_" to indicate that they are test functions.
  (func (export "wast_test_example1")
    ;; Test logic for example1
  )
  (func (export "wast_test_example2")
    ;; Test logic for example2
  )

  ;; WASI mocks (note that if a specific WASI API function is not mocked here, it will have a default implementation with a panic)
  (func $fd_write (param i32 i32 i32 i32) (result i32)
    ;; Mock implementation of fd_write here...
    ;; ...
  )
  (export "fd_write" (func $fd_write))
)

## References
- https://yurydelendik.github.io/webassembly-dwarf/
- https://mutants.rs/welcome.html
- https://rust-fuzz.github.io/book/introduction.html
- https://mutants.rs/getting-started.html
- https://microsoft.github.io/code-with-engineering-playbook/automated-testing/shadow-testing/