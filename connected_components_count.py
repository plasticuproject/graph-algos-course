"""connected_components_count.py"""

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

GRAPH = {
    "0": ["8", "1", "5"],
    "1": ["0"],
    "5": ["0", "8"],
    "8": ["0", "5"],
    "2": ["3", "4"],
    "3": ["2", "4"],
    "4": ["3", "2"],
}


# Depth first has-path
def depth_first_has_path_recursive(graph: dict, src: str,
                                   visited: set) -> bool:
    """Depth first has-path recursive algo with cyclical checks."""
    if src in visited:
        return False
    visited.add(src)
    for neighbor in graph[src]:
        depth_first_has_path_recursive(graph, neighbor, visited)
    return True


# Breadth first has-path
def breadth_first_has_path(graph: dict, src: str, visited: set) -> bool:
    """Breadth first has-path iterative algo with cyclical checks."""
    queue: list = [src]
    while queue:
        current: str = queue.pop(0)
        if current in visited:
            continue
        visited.add(current)
        for neighbor in graph[current]:
            queue.append(neighbor)
    return True


def connected_components_count(graph: dict) -> int:
    """Takes in a graph adjacency list, traverses the graph
    depth-first and returns the number of connected components."""
    count: int = 0
    visited: set = set()
    for node in graph:
        if depth_first_has_path_recursive(graph, node, visited):
            count += 1
    return count


def breadth_first_components_count(graph: dict) -> int:
    """Takes in a graph adjacency list, traverses the graph
    breadth first and returns the number of connected components."""
    count: int = 0
    visited: set = set()
    previous_visited: set = visited.copy()
    for node in graph:
        breadth_first_has_path(graph, node, visited)
        if previous_visited != visited:
            count += 1
        previous_visited = visited.copy()
    return count


# TESTS
assert connected_components_count(GRAPH) == 2
assert breadth_first_components_count(GRAPH) == 2
