"""undirected_path.py"""

# Our simple undirected graph structure to play with
"""
┌───┐     ┌───┐
│   │     │   │
│ i ├─────┤ j │
│   │     │   │
└─┬─┘     └───┘
  │
  │
  │
  │
  │
┌─┴─┐     ┌───┐
│   │     │   │
│ k ├─────┤ l │
│   │     │   │
└─┬─┘     └───┘
  │
  │
  │
  │
  │
┌─┴─┐
│   │
│ m │
│   │
└───┘





┌───┐     ┌───┐
│   │     │   │
│ o ├─────┤ n │
│   │     │   │
└───┘     └───┘
"""

# Edge list
EDGES = [
    ["i", "j"],
    ["k", "i"],
    ["m", "k"],
    ["k", "l"],
    ["o", "n"],
]

# Convert to adjacency list, used to test against
GRAPH = {
    "i": ["j", "k"],
    "j": ["i"],
    "k": ["i", "m", "l"],
    "m": ["k"],
    "l": ["k"],
    "o": ["n"],
    "n": ["o"],
}


# Helper function
def _build_graph(edges: list) -> dict:
    """Helper function to build undirected graph from edges."""
    graph: dict = {}
    _a: str
    _b: str
    for edge in edges:
        [_a, _b] = edge
        if _a not in graph:
            graph[_a] = []
        if _b not in graph:
            graph[_b] = []
        graph[_a].append(_b)
        graph[_b].append(_a)
    return graph


# Depth first has-path
def depth_first_has_path_recursive(graph: dict, src: str, dst: str,
                                   visited: set) -> bool:
    """Depth first has-path recursive algo with cyclical checks."""
    if src == dst:
        return True
    if src in visited:
        return False
    visited.add(src)
    for neighbor in graph[src]:
        if depth_first_has_path_recursive(graph, neighbor, dst, visited):
            return True
    return False


# Undirected graph traversal algo
def undirected_path(edges: list, node_a: str, node_b: str) -> bool:
    """Undirected depth first has-path recursive algo
    with cyclical checks from edge list input."""
    graph: dict = _build_graph(edges)
    return depth_first_has_path_recursive(graph, node_a, node_b, set())


# TESTS
TEST_GRAPH = _build_graph(EDGES)
assert TEST_GRAPH == GRAPH

assert undirected_path(EDGES, "j", "m")
assert undirected_path(EDGES, "j", "n") is False
