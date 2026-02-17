# Orbit B — Mirrored Layout (ASCII)

4 variants on inner orbit, actors + sub-agents distributed symmetrically left and right.

```text
+----------------------------------------------------------------------------------------------------------+
|                                                                                                          |
|   [ACTOR a1] opencode/server running               [ACTOR a3] opencode/server stopped                   |
|    +------+                                          +------+                                            |
|    | [sa] | frontend                                 | [sa] | cleanup                                    |
|    | [sa] | routing                                  +------+                                            |
|    | [sa] | +2 more                                      |                                               |
|    +------+                                              |                                               |
|        |                                                 |                                               |
|        |            +~~~~~~~~~~+       +~~~~~~~~~~+      |                                               |
|        +------------|  v1      |       |  v3      |------+                                               |
|                     |  clean   |       |  clean   |                                                      |
|                     +~~~~~~~~~~+       +~~~~~~~~~~+                                                      |
|                           \                 /                                                            |
|                            \               /                                                             |
|                         +================+                                                               |
|                         ||   PRODUCT    ||                                                               |
|                         ||  prd_2o02..  ||                                                               |
|                         ||  dirty  4v   ||                                                               |
|                         +================+                                                               |
|                            /               \                                                             |
|                           /                 \                                                            |
|                     +~~~~~~~~~~+       +~~~~~~~~~~+                                                      |
|                     |  v4      |       |  v2      |                                                      |
|                     |  dirty   |       |  dirty   |                                                      |
|                     +~~~~~~~~~~+       +~~~~~~~~~~+                                                      |
|                          |                  |                                                            |
|        +-----------------+                  +-----------------+                                          |
|        |                                                      |                                          |
|    +------+                                              +------+                                        |
|    | [sa] | review                                       | [sa] | test                                   |
|    | [sa] | verify                                       | [sa] | docs                                   |
|    +------+                                              +------+                                        |
|   [ACTOR a4] opencode/server idle                    [ACTOR a2] opencode/server running                  |
|                                                                                                          |
+----------------------------------------------------------------------------------------------------------+
```

_Orbit B mirrored: product core at center, 4 variants in diamond, actors and sub-agent mini-nodes balanced left/right._
