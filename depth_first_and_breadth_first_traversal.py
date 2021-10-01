"""depth_first_and_breadth_first_traversal.py"""
from collections import deque
from typing import Deque, Dict, List

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
GRAPH: Dict[str, List[str]] = {
    "a": ["c", "b"],
    "b": ["d"],
    "c": ["e"],
    "d": ["f"],
    "e": [],
    "f": [],
}


# Depth first traversal algos
def depth_first_print_iterative(graph: Dict[str, List[str]],
                                source: str) -> List[str]:
    """Depth first print iterative algo."""
    stack: Deque[str] = deque([source])
    r_list: List[str] = list()
    while stack:
        current: str = stack.pop()
        print(current)
        r_list.append(current)
        for neighbor in graph[current]:
            stack.append(neighbor)
    return r_list


def depth_first_print_recursive(graph: Dict[str, List[str]], source: str,
                                r_list: List[str]) -> List[str]:
    """Depth first print recursive algo."""
    print(source)
    r_list.append(source)
    for neighbor in graph[source]:
        depth_first_print_recursive(graph, neighbor, r_list)
    return r_list


# Breadth first traversal algo
def breadth_first_print_iterative(graph: Dict[str, List[str]],
                                  source: str) -> List[str]:
    """Breadth first print iterative algo."""
    queue: Deque[str] = deque([source])
    r_list: List[str] = list()
    while queue:
        current: str = queue.popleft()
        print(current)
        r_list.append(current)
        for neighbor in graph[current]:
            queue.append(neighbor)
    return r_list


# TESTS
TEST_GRAPH_1: List[str] = depth_first_print_iterative(GRAPH, "a")
assert TEST_GRAPH_1 == ["a", "b", "d", "f", "c", "e"]
print()

R_LIST: List[str] = list()
GRAPH_TEST_2 = depth_first_print_recursive(GRAPH, "a", R_LIST)
assert GRAPH_TEST_2 == ["a", "c", "e", "b", "d", "f"]
print()

GRAPH_TEST_3: List[str] = breadth_first_print_iterative(GRAPH, "a")
assert GRAPH_TEST_3 == ["a", "c", "b", "e", "d", "f"]
