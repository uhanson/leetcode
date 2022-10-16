#include <string.h>

struct ListNode {
    int val;
    struct ListNode *next;
};

void deleteNode(struct ListNode* node) {
    memmove(node, node->next, sizeof(struct ListNode));
}