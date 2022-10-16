#include <stdio.h>
#include <stdlib.h>

struct ListNode {
    int val;
    struct ListNode *next;
};

struct ListNode *create_list(int val, struct ListNode *next) {
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

struct ListNode* partition(struct ListNode* head, int x) {
    struct ListNode *less = NULL;
    struct ListNode *less_last = NULL;
    struct ListNode *more = NULL;
    struct ListNode *more_last = NULL;

    while (head != NULL) {
        if (head->val < x) {
            if (less_last == NULL) {
                less_last = head;
                less = head;
            } else {
                less_last->next = head;
                less_last = head;
            }
        } else {
            if (more_last == NULL) {
                more_last = head;
                more = head;
            } else {
                more_last->next = head;
                more_last = head;
            }
        }

        head = head->next;
    }

    if (less_last != NULL) less_last->next = more;
    if (more_last != NULL) more_last->next = NULL;

    return less != NULL ? less : more;
}

void main() {
    struct ListNode *list = create_list(1, create_list(4, create_list(3, create_list(2, create_list(5, create_list(2, NULL))))));

    print_list(list);
    list = partition(list, 3);
    print_list(list);
    destroy_list(list);
}