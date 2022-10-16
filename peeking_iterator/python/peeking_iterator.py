class Iterator(object):
    def __init__(self, nums):
        self.nums = nums

    def hasNext(self):
        return len(self.nums) > 0

    def next(self):
        ret = self.nums[0];

        self.nums = self.nums[1:]

        return ret
        
class PeekingIterator(object):
    def __init__(self, iterator):
        self.iter = iterator
        self.nextNum = None

    def peek(self):
        if self.nextNum is None:
            self.nextNum = self.iter.next()

        return self.nextNum

    def next(self):
        if self.nextNum is None:
            return self.iter.next()
        else:
            ret = self.nextNum
            self.nextNum = None

        return ret

    def hasNext(self):
        return self.iter.hasNext() or self.nextNum is not None

peekingIterator = PeekingIterator(Iterator([1, 2, 3])); # [1,2,3]
print(peekingIterator.next());    # return 1, the pointer moves to the next element [1,2,3].
print(peekingIterator.peek());    # return 2, the pointer does not move [1,2,3].
print(peekingIterator.next());    # return 2, the pointer moves to the next element [1,2,3]
print(peekingIterator.next());    # return 3, the pointer moves to the next element [1,2,3]
print(peekingIterator.hasNext()); # return False