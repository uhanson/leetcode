class ListNode(object):
    def __init__(self, x):
        self.val = x
        self.next = None

    def fromList(list):
        if list is not None and len(list) > 0:
            head = ListNode(list[0])
            last = head

            for v in list[1:]:
                node = ListNode(v)
                last.next = node
                last = node

            return head
        else:
            return None

class Solution(object):
    def getIntersectionNode(self, headA, headB):
        """
        :type head1, head1: ListNode
        :rtype: ListNode
        """

        nodes = {}
        while headA:
            nodes.add(headA.val)
            headA = headA.next

        while headB:
            if headB.val in nodes:
                return headB

        return None
        
print(Solution().getIntersectionNode(ListNode.fromList([1,9,1,2,4]), ListNode.fromList([3,2,4])))