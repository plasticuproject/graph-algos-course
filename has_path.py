"""has_path.py"""
from collections import deque
from typing import Deque, List, Dict

# Our simple acyclic graph structure to play with
"""
┌───┐     ┌───┐     ┌───┐
│   │     │   │     │   │
│ f ├────►│ g ├────►│ h │
│   │     │   │     │   │
└─┬─┘     └───┘     └───┘
  │         ▲
  │         │
  │     ┌───┘
  │     │
  ▼     │
┌───┐   │ ┌───┐
│   ├───┘ │   │
│ i │     │ j │
│   │◄────┤   │
└─┬─┘     └───┘
  │
  │
  │
  │
  ▼
┌───┐
│   │
│ k │
│   │
└───┘
"""
GRAPH: Dict[str, List[str]] = {
    "f": ["g", "i"],
    "g": ["h"],
    "h": [],
    "i": ["g", "k"],
    "j": ["i"],
    "k": [],
}


# Depth first has-path
def depth_first_has_path_recursive(graph: Dict[str, List[str]], src: str,
                                   dst: str) -> bool:
    """Depth first has-path recursive algo."""
    if src == dst:
        return True
    for neighbor in graph[src]:
        if depth_first_has_path_recursive(graph, neighbor, dst):
            return True
    return False


# Breadth first has-path
def breadth_first_has_path(graph: Dict[str, List[str]], src: str,
                           dst: str) -> bool:
    """Breadth first has-path iterative algo."""
    queue: Deque[str] = deque([src])
    while queue:
        current: str = queue.popleft()
        if current == dst:
            return True
        for neighbor in graph[current]:
            queue.append(neighbor)
    return False


# TESTS
assert depth_first_has_path_recursive(GRAPH, "f", "k")
assert depth_first_has_path_recursive(GRAPH, "j", "f") is False

assert breadth_first_has_path(GRAPH, "f", "k")
assert breadth_first_has_path(GRAPH, "j", "f") is False
