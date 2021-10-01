"""largest_component.py"""
from collections import deque
from typing import Deque, List, Dict, Set

# Our 2 component undirected graph structure to play with
"""
┌───┐     ┌───┐     ┌───┐
│   │     │   │     │   │
│ 1 ├─────┤ 0 ├─────┤ 5 │
│   │     │   │     │   │
└───┘     └─┬─┘     └─┬─┘
            │         │
            │         │
            │         │
            │         │
            │         │
          ┌─┴─┐       │
          │   │       │
          │ 8 ├───────┘
          │   │
          └───┘





┌───┐     ┌───┐
│   │     │   │
│ 2 ├─────┤ 3 │
│   │     │   │
└─┬─┘     └─┬─┘
  │         │
  │         │
  │         │
  │         │
  │         │
  │       ┌─┴─┐
  │       │   │
  └───────┤ 4 │
          │   │
          └───┘
"""

GRAPH: Dict[str, List[str]] = {
    "0": ["8", "1", "5"],
    "1": ["0"],
    "5": ["0", "8"],
    "8": ["0", "5"],
    "2": ["3", "4"],
    "3": ["2", "4"],
    "4": ["3", "2"],
}


# Depth first node count
def depth_first_count_recursive(graph: Dict[str, List[str]], src: str,
                                visited: Set[str]) -> int:
    """Depth first node count recursive algo with cyclical checks."""
    size: int = 1
    if src in visited:
        return 0
    visited.add(src)
    for neighbor in graph[src]:
        size += depth_first_count_recursive(graph, neighbor, visited)
    return size


# Breadth first node count
def breadth_first_count(graph: Dict[str, List[str]], src: str,
                        visited: Set[str]) -> int:
    """Breadth first node count iterative algo with cyclical checks."""
    queue: Deque[str] = deque([src])
    size: int = 0
    while queue:
        current: str = queue.popleft()
        if current in visited:
            continue
        visited.add(current)
        for neighbor in graph[current]:
            queue.append(neighbor)
        size += 1
    return size


# Largest components node count algos
def depth_first_largest_component(graph: Dict[str, List[str]]) -> int:
    """Takes in a graph adjacency list, traverses the graph
    depth-first and returns the number of nodes in the
    largest connected component."""
    largest: int = 0
    visited: Set[str] = set()
    size: int
    for node in graph:
        size: int = depth_first_count_recursive(graph, node, visited)
        if size > largest:
            largest = size
    return largest


def breadth_first_largest_component(graph: Dict[str, List[str]]) -> int:
    """Takes in a graph adjacency list, traverses the graph
    breadth-first and returns the number of nodes in the
    largest connected component."""
    largest: int = 0
    visited: Set[str] = set()
    size: int
    for node in graph:
        size: int = breadth_first_count(graph, node, visited)
        if size > largest:
            largest = size
    return largest


# TESTS
assert depth_first_largest_component(GRAPH) == 4
assert breadth_first_largest_component(GRAPH) == 4
