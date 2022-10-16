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
    printf("]");
}

struct ListNode* mergeKLists(struct ListNode **lists, int listsSize) {
    struct ListNode **cache = malloc(sizeof(struct ListNode*) * listsSize);
    int cacheSize = 0;

    for (int i = 0; i < listsSize; i++)
    {
        if (lists[i] != NULL) {
            cache[cacheSize] = lists[i];
            cacheSize++;
        }
    }

    struct ListNode *head = NULL;
    struct ListNode *tail = NULL;

    while (cacheSize > 0) {
        int min = INT_MAX;
        int min_i = 0;

        for (int i = cacheSize - 1; i >= 0; i--) {
            if (cache[i]->val < min) {
                min = cache[i]->val;
                min_i = i;
            }
        }

        if (tail == NULL) {
            tail = cache[min_i];
            head = tail;
        } else {
            tail->next = cache[min_i];
            tail = tail->next;
        }

        if (cache[min_i]->next == NULL) {
            cacheSize--;

            if (min_i < cacheSize) {
                cache[min_i] = cache[cacheSize];
            }
        } else {
            cache[min_i] = cache[min_i]->next;
        }
    }

    free(cache);

    return head;
}

void main() {
    struct ListNode* lists[] = {
        createNode(1, createNode(2, createNode(3, NULL))),
        createNode(4, createNode(5, createNode(6, NULL)))
    };

    struct ListNode *result = mergeKLists(lists, 2);

    printList(result);
    deleteList(result);
}