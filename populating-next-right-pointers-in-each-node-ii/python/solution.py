class Node(object):
    def __init__(self, val=0, left=None, right=None, next=None):
        self.val = val
        self.left = left
        self.right = right
        self.next = next

class Solution(object):
    def go(self, node, level, cache):
        print(cache)

        if node is not None:
            if level in cache:
                node.next = cache[level]

            cache[level] = node

            self.go(node.right, level + 1, cache)
            self.go(node.left, level + 1, cache)

    def connect(self, root):
        self.go(root, 0, {})
        return root

def toListNext(node):
    ret = []

    if node is not None:
        ret.append(node.val)

        next = node.right if node.left is None else node.left

        while node.next is not None:
            node = node.next

            ret.append(node.val)

            if next is None:
                next = node.right if node.left is None else node.left

        ret += ["#"] + toListNext(next)

    return ret

def toListTree(root):
    ret = []

    if root is not None:
        nodes = [root]

        while any(nodes):
            next_nodes = []

            for node in nodes:
                if node is not None:
                    ret.append(node.val)
                    next_nodes.append(node.left)
                    next_nodes.append(node.right)

            nodes = next_nodes                    

    return ret                

def fromList(list):
    if len(list) == 0:
        return None
    else:
        root = Node(list[0])
        list = list[1:]

        nodes = [root]
        for node in nodes:
            if len(list) > 0:
                if list[0] is not None:
                    node.left = Node(list[0])
                nodes.append(node.left)
                list = list[1:]

                if len(list) > 0:
                    node.right = Node(list[0])
                nodes.append(node.right)
                list = list[1:]

        return root

def main():
    print(toListTree(fromList([1, 2, 3, 4, None, 6])))
    print(toListNext(Solution().connect(fromList([1, 2, 3, 4, None, 6]))))

main()