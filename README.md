# Rust Concurrency Projects

This repository contains a collection of small projects written in Rust, designed to explore and practice concurrency concepts. Each project demonstrates different concurrency mechanisms in Rust, from threading to networked communication, and covers topics like barriers, semaphores, and gRPC.

## Projects Overview
### 1. Sieve

The Sieve project implements the Sieve of Eratosthenes algorithm, an efficient method for finding all prime numbers up to a specified limit. This implementation leverages Rust’s concurrency capabilities to optimize the algorithm by distributing tasks across threads.
- sieve
	- Purpose: Prime number generation using a concurrent approach to the Sieve of Eratosthenes.

### 2. Sockets

The Sockets folder contains several TCP-based applications, including implementations of the master-worker pattern, a basic calculator, and a multi-client chat application.

- master_worker_tcp_pi:
	- Implements a master-worker model over TCP to approximate the value of pi. The master node distributes computational tasks to worker nodes, which return partial results for aggregation.

- calculator:
   - A simple TCP server that performs basic arithmetic operations (addition, subtraction, multiplication, and division). Clients can connect via TCP to request calculations.

- chat_with_multiple_sockets:
   - A command-line-based chat server that allows multiple clients to communicate. This project demonstrates handling multiple TCP connections concurrently.

### 3. gRPC

The gRPC folder contains projects that use gRPC for communication between clients and servers, demonstrating an alternative to TCP for client-server interactions.

- cachedPi:
	- A gRPC server that allows clients to approximate the value of pi. Results are cached to improve performance for repeated requests.

- calc:
	- A gRPC-based implementation of the calculator server (similar to the TCP calculator), which supports addition, subtraction, multiplication, and division.

### 4. Barriers

The Barriers folder contains examples demonstrating the use of barriers, with two implementations showcasing different synchronization methods.

- barrier_lock_cond:
	- A barrier example that uses lock conditions for synchronization.

- barrier_sem:
	- A barrier example that uses semaphores for synchronization.

### 5. Atomics

The Atomics folder contains examples of the producer-consumer pattern, illustrating two approaches to handling shared data between threads using atomic operations.

- prod_con_sem_mux:
	- A producer-consumer example that uses semaphores and mutexes for thread synchronization.

- prod_con_lock_cond:
	- A producer-consumer example that uses lock conditions for thread synchronization.

### 6. Num Int

The Num Int project uses a recursive thread approach to approximate the value of pi. This project demonstrates parallelization by recursively dividing tasks among threads, with each thread performing a portion of the calculation.

- Folder: num_int
	- Purpose: Pi approximation using recursive threads.

## Getting Started

To run any of these projects, make sure you have Rust installed. Clone this repository and navigate to the desired project folder.

```
# Clone the repository
git clone <repository_url>
cd <project_folder>
```

Build and run each project using cargo or rustc:

```
cargo build --release
cargo run
```
or
```
rustc <file_name>
```

## Contributing

Contributions, bug reports, and feature requests are welcome! Feel free to fork this repository and open a pull request.
