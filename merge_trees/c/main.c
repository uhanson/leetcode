#include <stdlib.h>

struct TreeNode {
    int val;
    struct TreeNode *left;
    struct TreeNode *right;
};

struct TreeNode *createTreeNode(int val, struct TreeNode *left, struct TreeNode *right) {
    struct TreeNode *node = malloc(sizeof(struct TreeNode));

    node->val = val;
    node->left = left;
    node->right = right;

    return node;
}

struct TreeNode *mergeTrees(struct TreeNode *root1, struct TreeNode *root2) {
    if (root1 && root2) {
        struct TreeNode *node = malloc(sizeof(struct TreeNode));

        node->val = root1->val + root2->val;
        node->left = mergeTrees(root1->left, root2->left);
        node->right = mergeTrees(root1->right, root2->right);

        return node;
    }
    
    if (root1) {
        return root1;
    }

    return root2;
}

void main() {

}