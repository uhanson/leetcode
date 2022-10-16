#include <stdlib.h>

struct TreeNode {
    int val;
    struct TreeNode *left;
    struct TreeNode *right;
};

struct TreeNode* searchBST(struct TreeNode* root, int val) {
    if (root == NULL) return NULL;
    if (root->val == val) return root;

    struct TreeNode *ret = searchBST(root->left, val);

    if (ret == NULL) {
        ret = searchBST(root->right, val);
    }

    return ret;
}