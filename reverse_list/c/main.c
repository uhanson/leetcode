#include <stdio.h>
#include <stdlib.h>

struct ListNode {
    int val;
    struct ListNode *next;
};

struct ListNode *create_node(int val, struct ListNode *next) {
    struct ListNode *list = malloc(sizeof(struct ListNode));

    list->val = val;
    list->next = next;

    return list;
}

void destroy_list(struct ListNode *list) {
    if (list != NULL) {
        destroy_list(list->next);
        free(list);
    }
}

void print_list(struct ListNode *list) {
    printf("[");

    while (list != NULL) {
        printf("%d", list->val);

        if (list->next != NULL) printf(",");

        list = list->next;
    }

    printf("]\n");
}

struct ListNode* reverse(struct ListNode *node, struct ListNode *next) {
    if (node == NULL) return next;

    struct ListNode *next_node = node->next;

    node->next = next;

    return reverse(next_node, node);
}

struct ListNode* reverseList(struct ListNode* head) {
    return reverse(head, NULL);
}

void main() {
    struct ListNode *list = create_node(1, create_node(2, create_node(3, create_node(4, create_node(5, create_node(6, NULL))))));

    print_list(list);

    list = reverseList(list);

    print_list(list);
    destroy_list(list);
}