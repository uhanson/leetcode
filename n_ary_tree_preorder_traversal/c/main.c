#include <stdio.h>
#include <stdlib.h>

struct Node {
    int val;
    int numChildren;
    struct Node** children;
};

struct Node *createNode(int val, int numChildren, struct Node **children) {
    struct Node *node = malloc(sizeof(struct Node));

    node->val = val;
    node->numChildren = numChildren;
    node->children = children;

    return node;
}

void deleteNode(struct Node *node) {
    struct Node **children = node->children;

    for (int i = node->numChildren; i > 0; i--) {
        deleteNode(*children++);
    }
}

int *fill(struct Node *node, int *retPtr) {
    if (node != NULL) {
        *retPtr++ = node->val;

        for (int currChild = 0; currChild < node->numChildren; currChild++) {
            retPtr = fill(node->children[currChild], retPtr);
        }
    }

    return retPtr;
}

int* preorder(struct Node* root, int* returnSize) {
    int *ret = malloc(sizeof(int) * 10000);
    int *retPtr = ret;

    *returnSize = fill(root, retPtr) - ret;

    return ret;
}

void main() {
    struct Node *nodes[] = { createNode(2, 0, NULL), createNode(3, 0, NULL) };
    struct Node *root = createNode(1, 2, nodes);

    int returnSize;
    int *list = preorder(root, &returnSize);

    deleteNode(root);

    int *listPtr = list;

    while (returnSize-- > 0) {
        printf("%d ", *listPtr++);
    }

    printf("\n");
    free(list);

    return;
}