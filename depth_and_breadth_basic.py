"""depth_and_breadth_basic.py"""

# Our simple graph structure to play with
GRAPH = {
    'a': ['c', 'b'],
    'b': ['d'],
    'c': ['e'],
    'd': ['f'],
    'e': [],
    'f': [],
}


# Depth First Traversal algos
def depth_first_print_iterative(graph: dict, source: str) -> list:
    """Depth First print interative algo."""
    stack: list = [source]
    current: str
    r_list: list = []
    while stack:
        current = stack.pop()
        print(current)
        r_list.append(current)
        for neighbor in graph[current]:
            stack.append(neighbor)
    return r_list


def depth_first_print_recursive(graph: dict, source: str,
                                r_list: list) -> list:
    """Depth First print recursive algo."""
    print(source)
    r_list.append(source)
    for neighbor in graph[source]:
        depth_first_print_recursive(graph, neighbor, r_list)
    return r_list


# Breadth First Traversal algo
def breadth_first_print_iterative(graph: dict, source: list) -> list:
    """Breadth First print iterative algo."""
    queue: list = [source]
    current: str
    r_list: list = []
    while queue:
        current = queue.pop(0)
        print(current)
        r_list.append(current)
        for neighbor in graph[current]:
            queue.append(neighbor)
    return r_list


# Tests
assert depth_first_print_iterative(GRAPH,
                                   'a') == ['a', 'b', 'd', 'f', 'c', 'e']
print()
R_LIST: list = []
assert depth_first_print_recursive(GRAPH, 'a',
                                   R_LIST) == ['a', 'c', 'e', 'b', 'd', 'f']
print()
assert breadth_first_print_iterative(GRAPH,
                                     'a') == ['a', 'c', 'b', 'e', 'd', 'f']
