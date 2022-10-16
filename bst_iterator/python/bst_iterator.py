# Definition for a binary tree node.
# class TreeNode(object):
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class BSTIterator(object):

    def __init__(self, root):
        self.stack = []
        self.traverse(root)

    def traverse(self, node):
        while node != None:
            self.stack.append(node)
            node = node.left

    def next(self):
        if len(self.stack) > 0:
            node = self.stack.pop()
            
            self.traverse(node.right)

            return node.val
        else:
            raise Exception("No more nodes to traverse")
        

    def hasNext(self):
        return len(self.stack) > 0


# Your BSTIterator object will be instantiated and called as such:
# obj = BSTIterator(root)
# param_1 = obj.next()
# param_2 = obj.hasNext()