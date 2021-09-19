"""depth_first_and_breadth_first_traversal.py"""

# Our simple graph structure to play with
"""
┌───┐     ┌───┐
│   │     │   │
│ a ├────►│ b │
│   │     │   │
└─┬─┘     └─┬─┘
  │         │
  │         │
  │         │
  │         │
  ▼         ▼
┌───┐     ┌───┐
│   │     │   │
│ c │     │ d │
│   │     │   │
└─┬─┘     └─┬─┘
  │         │
  │         │
  │         │
  │         │
  ▼         ▼
┌───┐     ┌───┐
│   │     │   │
│ e │     │ f │
│   │     │   │
└───┘     └───┘
"""
GRAPH = {
    "a": ["c", "b"],
    "b": ["d"],
    "c": ["e"],
    "d": ["f"],
    "e": [],
    "f": [],
}


# Depth first traversal algos
def depth_first_print_iterative(graph: dict, source: str) -> list:
    """Depth first print iterative algo."""
    stack: list = [source]
    r_list: list = []
    while stack:
        current: str = stack.pop()
        print(current)
        r_list.append(current)
        for neighbor in graph[current]:
            stack.append(neighbor)
    return r_list


def depth_first_print_recursive(graph: dict, source: str,
                                r_list: list) -> list:
    """Depth first print recursive algo."""
    print(source)
    r_list.append(source)
    for neighbor in graph[source]:
        depth_first_print_recursive(graph, neighbor, r_list)
    return r_list


# Breadth first traversal algo
def breadth_first_print_iterative(graph: dict, source: list) -> list:
    """Breadth first print iterative algo."""
    queue: list = [source]
    r_list: list = []
    while queue:
        current: str = queue.pop(0)
        print(current)
        r_list.append(current)
        for neighbor in graph[current]:
            queue.append(neighbor)
    return r_list


# TESTS
assert depth_first_print_iterative(GRAPH,
                                   "a") == ["a", "b", "d", "f", "c", "e"]
print()
R_LIST: list = []
assert depth_first_print_recursive(GRAPH, "a",
                                   R_LIST) == ["a", "c", "e", "b", "d", "f"]
print()
assert breadth_first_print_iterative(GRAPH,
                                     "a") == ["a", "c", "b", "e", "d", "f"]
