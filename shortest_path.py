"""shortest_path.py"""

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
EDGES = [
    ["w", "x"],
    ["x", "y"],
    ["z", "y"],
    ["z", "v"],
    ["w", "v"],
]

# Convert to adjacency list, used to test against
GRAPH = {
    "w": ["x", "v"],
    "x": ["w", "y"],
    "y": ["x", "z"],
    "z": ["y", "v"],
    "v": ["z", "w"],
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


# Breadth first edge count
def breadth_first_count(graph: dict, src: str, dst: str, visited: set) -> int:
    """Breadth first edge count iterative algo with cyclical checks.
    Returns shortest amount of edgest between two nodes, or -1 if
    components are not connected."""
    queue: list = [[src, 0]]
    while queue:
        current, distance = queue.pop(0)
        if current == dst:
            return distance
        for neighbor in graph[current]:
            if neighbor in visited:
                continue
            visited.add(neighbor)
            queue.append([neighbor, distance + 1])
    return -1


# Depth first edge count
def depth_first_count_iterative(graph: dict, src: str, dst: str,
                                visited: set) -> int:
    """Depth first edge count recursive algo with cyclical checks.
    Returns shortest amount of edgest between two nodes, or -1 if
    components are not connected."""
    stack: list = [[src, 0]]
    while stack:
        current, distance = stack.pop()
        if current == dst:
            return distance
        visited.add(src)
        for neighbor in graph[current]:
            if neighbor in visited:
                continue
            visited.add(neighbor)
            stack.append([neighbor, distance + 1])
    return -1


def breadth_first_shortest_path(edges: list, node_a: str, node_b: str) -> int:
    """Takes in a graph adjacency list, traverses the graph
    breadth-first and returns the number of edges from the
    source node to the destination node, or -1 if components
    are not connected."""
    graph = _build_graph(edges)
    visited: set = set([node_a])
    shortest_path = breadth_first_count(graph, node_a, node_b, visited)
    return shortest_path


def depth_first_shortest_path(edges: list, node_a: str, node_b: str) -> int:
    """Takes in a graph adjacency list, traverses the graph
    depth-first and returns the number of edges from the
    source node to the destination node, or -1 if components
    are not connected."""
    graph = _build_graph(edges)
    visited: set = set([node_a])
    # visited: set = set()
    shortest_path = depth_first_count_iterative(graph, node_a, node_b, visited)
    return shortest_path


# TESTS
assert _build_graph(EDGES) == GRAPH
assert breadth_first_shortest_path(EDGES, "w", "z") == 2
assert depth_first_shortest_path(EDGES, "w", "z") == 2
