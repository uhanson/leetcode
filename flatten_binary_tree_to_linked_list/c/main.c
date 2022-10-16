#include <stdio.h>
#include <stdlib.h>

struct TreeNode {
    int val;
    struct TreeNode *left;
    struct TreeNode *right;
};

void delete_tree(struct TreeNode *tree) {
    if (tree != NULL) {
        delete_tree(tree->left);
        delete_tree(tree->right);
        free(tree);
    }
}

struct TreeNode *create_tree(int val, struct TreeNode *left, struct TreeNode *right) {
    struct TreeNode *tree = malloc(sizeof(struct TreeNode));

    tree->val = val;
    tree->left = left;
    tree->right = right;

    return tree;
}

struct TreeNode* traverse(struct TreeNode *node) {
    if (node != NULL) {
        struct TreeNode *last = traverse(node->left);
        struct TreeNode *next = traverse(node->right);

        if (last != NULL) {
            last->right = node->right;
            node->right = node->left;
        }

        node->left = NULL;

        return next ? next : (last ? last : node);
    } else {
        return NULL;
    }
}

void flatten(struct TreeNode* root) {
    traverse(root);
}

void print_tree(struct TreeNode *node, int indent) {
    int c = indent;

    while (c-- > 0) printf("   ");

    if (node != NULL) {
        printf("%d\n", node->val);

        if (node->left != NULL || node->right != NULL)
        {
            print_tree(node->left, indent + 1);
            print_tree(node->right, indent + 1);
        }
    } else {
        printf("-\n");
    }
}

void main() {
    struct TreeNode* root = create_tree(1, 
                                create_tree(2, 
                                    create_tree(3, NULL, NULL), 
                                    create_tree(4, NULL, NULL)),
                                create_tree(5, 
                                    NULL, 
                                    create_tree(6, NULL, NULL)));

    print_tree(root, 0);
    flatten(root);
    print_tree(root, 0);
    delete_tree(root);
}