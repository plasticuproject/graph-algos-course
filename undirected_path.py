"""undirected_path.py"""
from collections import deque
from typing import Deque, List, Dict, Set

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
EDGES: List[List[str]] = [
    ["i", "j"],
    ["k", "i"],
    ["m", "k"],
    ["k", "l"],
    ["o", "n"],
]

# Convert to adjacency list, used to test against
GRAPH: Dict[str, List[str]] = {
    "i": ["j", "k"],
    "j": ["i"],
    "k": ["i", "m", "l"],
    "m": ["k"],
    "l": ["k"],
    "o": ["n"],
    "n": ["o"],
}


# Helper function
def _build_graph(edges: List[List[str]]) -> Dict[str, List[str]]:
    """Helper function to build undirected graph from edges."""
    graph: Dict[str, List[str]] = dict()
    _a: str
    _b: str
    for edge in edges:
        _a, _b = edge
        if _a not in graph:
            graph[_a] = list()
        if _b not in graph:
            graph[_b] = list()
        graph[_a].append(_b)
        graph[_b].append(_a)
    return graph


# Depth first has-path
def depth_first_has_path_recursive(graph: Dict[str, List[str]], src: str,
                                   dst: str, visited: Set[str]) -> bool:
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


# Breadth first has-path
def breadth_first_has_path(graph: Dict[str, List[str]], src: str, dst: str,
                           visited: Set[str]) -> bool:
    """Breadth first has-path iterative algo with cyclical checks."""
    queue: Deque[str] = deque([src])
    while queue:
        current: str = queue.popleft()
        if current == dst:
            return True
        if current in visited:
            continue
        visited.add(current)
        for neighbor in graph[current]:
            queue.append(neighbor)
    return False


# Undirected graph traversal algos
def undirected_path_depth_first(edges: List[List[str]], node_a: str,
                                node_b: str) -> bool:
    """Undirected depth first has-path recursive algo
    with cyclical checks from edge list input."""
    graph: Dict[str, List[str]] = _build_graph(edges)
    return depth_first_has_path_recursive(graph, node_a, node_b, set())


def undirected_path_breadth_first(edges: List[List[str]], node_a: str,
                                  node_b: str) -> bool:
    """Undirected breadth first has-path iterative algo
    with cyclical checks from edge list input."""
    graph: Dict[str, List[str]] = _build_graph(edges)
    return breadth_first_has_path(graph, node_a, node_b, set())


# TESTS
TEST_GRAPH: Dict[str, List[str]] = _build_graph(EDGES)
assert TEST_GRAPH == GRAPH

assert undirected_path_depth_first(EDGES, "j", "m")
assert undirected_path_depth_first(EDGES, "j", "n") is False

assert undirected_path_breadth_first(EDGES, "j", "m")
assert undirected_path_breadth_first(EDGES, "j", "n") is False
