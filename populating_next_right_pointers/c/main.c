#include <stdlib.h>

struct Node {
    int val;
    struct Node *left;
    struct Node *right;
    struct Node *next;
};

void traverse(struct Node *node, struct Node *stack[], int depth) {
    if (node) {
        node->next = stack[depth];
        stack[depth] = node;

        if (node->right) {
            traverse(node->right, stack, depth + 1);
        }

        if (node->left) {
            traverse(node->left, stack, depth + 1);
        }
    }
}

struct Node *connect(struct Node *root) {
	struct Node *stack[12] = {NULL};

    traverse(root, stack, 0);

    return root;
}