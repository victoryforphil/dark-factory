# Notes

- Use moon and proto to manage tools and deps
- Use a shared schemas language to define our types / API's
    - This will allow us to port to a new language eaier
    - Prisma was first choice but lacks solid rust support
    - Protobuf using elysia-protobuf seems like a good option
        - Libray is a bit immature - so we forked it to allow patches
- Bun + Elysia JS based server for Gen 1 API for the following reasons:
    - Faster iteration
    - Nice OpenAPI support which will is nice for testing and open source development
    - Out of the box plugins for stuff like llms.txt - userful for the spirit
    - In theroy - can do ProtoBuff -> OpenAPI -> Routes / Docs all out of the box.
- High level architecture is a core service that handles world / env / actor tracking and provides an API for agentic comms. The core service will be used by the agents to interact with the world and each other. The core service will also handle logging and other helpers.
- Core acts a spawnable api server that reads from a local database oof our stored world / env / actor state. It exposes HTTP/WebSocket APIs to query / manipulate this state, as well as an API for agentic comms. The core service is designed to be lightweight and fast, allowing for quick iteration and development.
- Frontend (first one being a simple CLI, followed by a TUI) will interact with the core service to provide a user interface for monitoring and interacting with the world and agents. The frontend will also provide tools for debugging and visualizing the state of the world and agents.


# Scope - Stage 0 
- [ ] Shared Schema REST API w/ Build System
- [ ] Exposed REST API Docs
- [ ] Schemas for:
    - [ ] World Definitions
    - [ ] Actor State (Spawned Agents and their info)
- [ ] REST API for Creating Worlds w/ a single directory
- [ ] REST API for Spawning OpenCode-based session servers in a defined world
    - [ ] OpenCode over its Server API based manager
- [ ] REST API for querying last known state of spawned agents
- [ ] Single world - single instance - single OpenCode agent for now
- [ ] Basic Rust-based CLI for interacting with the core service and querying state
