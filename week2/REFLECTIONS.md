# Reflections

## TOC
- [Safe vs. Unsafe Rust](#safe-vs-unsafe-rust)
- [Ownership and Lifetime](#ownership-and-lifetime)
- [Ownership Based Resource Management (OBRM)](#ownership-based-resource-management)

## Safe vs. Unsafe Rust

### What are the two programming languages contained within Rust?
Rust contains two programming languages: `Safe Rust` and `Unsafe Rust`. 
Safe Rust enforces strict safety guarantees at compile time, while Unsafe Rust allows for operations that can potentially violate these guarantees but provides more control over low-level details.

### When might a programmer need to use Unsafe Rust instead of Safe Rust?
A programmer might need to use Unsafe Rust in situations where performance is critical, when interfacing directly with hardware or operating systems, or when implementing low-level abstractions that Safe Rust cannot express. Unsafe Rust is used to perform operations that require fine-grained control over memory and other resources.

### What safety guarantees does Safe Rust provide?
Safe Rust provides several important safety guarantees, including:
- No dangling pointers
- No use-after-free errors
- No undefined behavior

### How does Unsafe Rust relate to Safe Rust?
Unsafe Rust adheres to the same rules and semantics as Safe Rust but allows for additional operations that are not checked by the compiler for safety. Unsafe Rust can perform raw pointer dereferencing, call unsafe functions, and perform other low-level operations. Despite this, it is designed to be as minimal as possible, enabling programmers to leverage unsafe features without losing the benefits of Rust's safety guarantees.

### What is the value of separating safe and unsafe languages in Rust?
Separating safe and unsafe languages in Rust allows developers to gain the benefits of unsafe control and performance optimizations without integrating a completely different language. This separation helps maintain the safety and reliability of Safe Rust while providing the flexibility needed for low-level system programming. It enables a clear distinction between safe and potentially unsafe code, making it easier to audit and maintain.


## Ownership and Lifetime

### What are the ownership rules in Rust?

Each value in Rust has a single owner. There can only be one owner at any given time, and when the owner goes out of
scope, the value is dropped.

### How does ownership help manage memory safely in Rust?

Ownership ensures memory is adequately cleaned up when the owner goes out of scope. 
This automatic cleanup prevents memory leaks and dangling pointers, leading to safer memory management 
without the need for a garbage collector.

### What is the difference between a deep copy and a shallow copy?

- **Deep Copy**: A deep copy duplicates the inner data of a value, creating a new instance with its own copies of the
  original value's data.
- **Shallow Copy**: A shallow copy only duplicates the pointer or metadata, not the actual data it points to. Both the
  original and the copy refer to the same underlying data.

### When are moves or copies inexpensive in Rust?

Moves or copies are inexpensive for types with a known, fixed size that are stored on the stack, such as integers.

### What is the Copy trait, and when can it be used?

The `Copy` trait can be used for types that are stored entirely on the stack and have a known, fixed size. Types that
implement the `Copy` trait can be copied instead of moved, meaning that after assignment, both the original and the copy
can be used without restrictions. Examples include integers, floating-point numbers, and tuples of `Copy` types.

### How does Rust's ownership model compare to garbage collection in other languages? What are the tradeoffs?

Rust's ownership model provides memory safety without the overhead of a garbage collector. Unlike garbage-collected
languages, Rust's compile-time checks ensure that memory is released as soon as it is no longer needed. The tradeoffs
include a steeper learning curve and more explicit memory management by the programmer, but the benefits are
deterministic performance and reduced runtime overhead.

### Why do you think Rust favors moves over deep copying by default? What are the advantages?

Rust favors moves over deep copying to improve performance and efficiency. Moves are generally cheaper than deep copies
because they involve transferring ownership without duplicating the data. This reduces the computational overhead and
memory usage, leading to faster and more efficient code.

### How does ownership affect how you design and structure programs in Rust? What changes compared to other languages?

Ownership affects program design by encouraging more explicit data ownership and transfer patterns. Compared to other
languages, Rust programs often have clearer resource management and fewer runtime surprises related to memory usage.


## Ownership Based Resource Management

**aka**: Resource acquisition is initialization (RAII)

### What is ownership-based resource management (OBRM) in Rust?
Ownership-based resource management (OBRM) in Rust is a system for managing resources by tying their lifetimes to the lifetimes of objects. This ensures that resources are properly acquired and released in a deterministic manner.

### What is the typical pattern for OBRM in Rust?
The typical pattern for OBRM in Rust involves creating an object to acquire a resource and allowing the Rust ownership system to automatically destroy the object to release the resource when it goes out of scope. This pattern ensures resources are properly cleaned up without requiring manual intervention.

### What is the most common resource managed by OBRM in Rust?
The most common resource managed by OBRM in Rust is memory. Rust uses types like `Box` and `Rc` to handle memory allocation and deallocation safely and efficiently, leveraging ownership semantics to ensure proper resource management.

### What other system resources can be managed with OBRM besides memory?
Besides memory, other system resources that can be managed with OBRM include:
- Threads
- Files
- Sockets
- Network connections
- Database connections
  OBRM can be applied to any resource that requires careful acquisition and release.

### How does OBRM help control resources in Rust since it lacks a garbage collector?
OBRM helps control resources in Rust by providing explicit acquire and release control. Since Rust lacks a garbage collector, OBRM ensures that resources are released as soon as they are no longer needed, based on the ownership and lifetime of objects. This deterministic resource management prevents resource leaks and ensures efficient use of system resources.