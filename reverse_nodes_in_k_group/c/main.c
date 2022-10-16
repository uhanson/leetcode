#include <limits.h>
#include <stdio.h>
#include <stdlib.h>

struct ListNode {
    int val;
    struct ListNode *next;
};

struct ListNode *createNode(int val, struct ListNode *next) {
    struct ListNode *node = (struct ListNode*) malloc(sizeof(struct ListNode));

    node->val = val;
    node->next = next;

    return node;
}

void deleteList(struct ListNode *list) {
    if (list != NULL) {
        deleteList(list->next);
        free(list);
    }
}

void printList(struct ListNode *list) {
    printf("[");
    while (list != NULL) {
        printf("%d", list->val);

        if (list->next) {
            printf(", ");
        }

        list = list->next;
    }
    printf("]\n");
}

int listSizeEnough(struct ListNode *head, int k) {
    struct ListNode *node = head;

    for (int i = 0; i < k; i++) {
        if (node == NULL) { return 0;}
        node = node->next;
    }

    return 1;
}

struct ListNode* reverseKGroup(struct ListNode* head, int k) {
    if (k == 1 || !listSizeEnough(head, k)) {
        return head;
    }

    struct ListNode *tail = NULL;
    struct ListNode *node = head;

    for (int i = 0; i < k; i++) {
        if (node != NULL) {
            struct ListNode *next = node->next;
            node->next = tail;
            tail = node;
            node = next;
        } else {
            break;
        }
    }

    head->next = reverseKGroup(node, k);

    return tail;
}

void main() {
    struct ListNode* list = createNode(1, createNode(2, createNode(3, createNode(4, createNode(5, createNode(6, NULL))))));
    struct ListNode *result = reverseKGroup(list, 2);

    printList(result);
    deleteList(result);
}