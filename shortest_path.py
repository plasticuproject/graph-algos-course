"""shortest_path.py"""
from collections import deque
from typing import Deque, List, Dict, Set, Union

# Our simple undirected graph structure to play with
"""
┌───┐     ┌───┐     ┌───┐
│   │     │   │     │   │
│ w ├─────┤ x ├─────┤ y │
│   │     │   │     │   │
└─┬─┘     └───┘     └─┬─┘
  │                   │
  │                   │
  │                   │
  │                   │
  │                   │
  │       ┌───┐     ┌─┴─┐
  │       │   │     │   │
  └───────┤ v ├─────┤ z │
          │   │     │   │
          └───┘     └───┘
"""

# Edge list
EDGES: List[List[str]] = [
    ["w", "x"],
    ["x", "y"],
    ["z", "y"],
    ["z", "v"],
    ["w", "v"],
]

# Convert to adjacency list, used to test against
GRAPH: Dict[str, List[str]] = {
    "w": ["x", "v"],
    "x": ["w", "y"],
    "y": ["x", "z"],
    "z": ["y", "v"],
    "v": ["z", "w"],
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


# Breadth first edge count
def breadth_first_count(graph: Dict[str, List[str]], src: str, dst: str,
                        visited: Set[str]) -> int:
    """Breadth first edge count iterative algo with cyclical checks.
    Returns shortest amount of edgest between two nodes, or -1 if
    components are not connected."""
    queue: Deque[List[Union[str, int]]] = deque([[src, 0]])
    while queue:
        current, distance = queue.popleft()
        if current == dst:
            return int(distance)
        for neighbor in graph[str(current)]:
            if neighbor in visited:
                continue
            visited.add(neighbor)
            queue.append([neighbor, int(distance) + 1])
    return -1


# Depth first edge count
def depth_first_count_iterative(graph: Dict[str, List[str]], src: str,
                                dst: str, visited: Set[str]) -> int:
    """Depth first edge count recursive algo with cyclical checks.
    Returns shortest amount of edgest between two nodes, or -1 if
    components are not connected."""
    stack: Deque[List[Union[str, int]]] = deque([[src, 0]])
    while stack:
        current, distance = stack.pop()
        if current == dst:
            return int(distance)
        for neighbor in graph[str(current)]:
            if neighbor in visited:
                continue
            visited.add(neighbor)
            stack.append([neighbor, int(distance) + 1])
    return -1


def breadth_first_shortest_path(edges: List[List[str]], node_a: str,
                                node_b: str) -> int:
    """Takes in a graph adjacency list, traverses the graph
    breadth-first and returns the number of edges from the
    source node to the destination node, or -1 if components
    are not connected."""
    graph: Dict[str, List[str]] = _build_graph(edges)
    visited: Set[str] = set(node_a)
    shortest_path: int = breadth_first_count(graph, node_a, node_b, visited)
    return shortest_path


def depth_first_shortest_path(edges: List[List[str]], node_a: str,
                              node_b: str) -> int:
    """Takes in a graph adjacency list, traverses the graph
    depth-first and returns the number of edges from the
    source node to the destination node, or -1 if components
    are not connected."""
    graph: Dict[str, List[str]] = _build_graph(edges)
    visited: Set[str] = set(node_a)
    shortest_path: int = depth_first_count_iterative(graph, node_a, node_b,
                                                     visited)
    return shortest_path


# TESTS
assert _build_graph(EDGES) == GRAPH
assert breadth_first_shortest_path(EDGES, "w", "z") == 2
assert depth_first_shortest_path(EDGES, "w", "z") == 2
