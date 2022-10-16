# Definition for a Node.
class Node(object):
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children

class Solution(object):
    def levelOrder(self, root):
        """
        :type root: Node
        :rtype: List[List[int]]
        """
   
        def collect(node, list):
            if ret.len() < level:
                ret.append([node.val])

            for child in node.children:
                ret[level].append(child.val)

        ret = list()

        if 
        ret.append([ret.val])