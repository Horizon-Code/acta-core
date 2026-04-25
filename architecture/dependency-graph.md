# Dependency Graph

Allowed conceptual dependency directions:

- `core` depends on architecture, not on profiles.
- `profiles` depend on core and architecture.
- `decisions` can constrain architecture/core/profiles.
- `research` can reference anything but must not act as normative source.
- `roadmap` can reference all layers for planning.
