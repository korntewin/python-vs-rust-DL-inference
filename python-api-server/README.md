# Python API Server project

This project is architectured in **Hexagonal Architecture** and **Domain Driven Design** approach for code scalability.

```text
api/            # interfacing code
    rest/       # REST API interface (there can be like CLI, Queue Consumer, etc.)
services/       # domain logic code
    adapters/   # abstract class for logic that need to interact with real-world
    <name>_service.py   # domain logic, splitted by domain
```