# Rust API Server project

This project is architectured in **Hexagonal Architecture** and **Domain Driven Design** approach for code scalability.

```text
src/
    api/            # interfacing code
    services/       # domain logic code
        adapters/   # abstract class for logic that need to interact with real-world
        <name>_service.rs   # domain logic, splitted by domain
```