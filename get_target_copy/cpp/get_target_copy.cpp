#include <stddef.h>

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;

    TreeNode(int x) : val(x), left(NULL), right(NULL) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

TreeNode* findNode(TreeNode* node, TreeNode* target) {
    if (node->val == target->val) {
        return node;
    } 
    
    if (node->left != NULL) {
        TreeNode* ret = findNode(node->left, target);

        if (ret != NULL) {
            return ret;
        }
    }
    
    if (node->right != NULL) {
        TreeNode* ret = findNode(node->right, target);

        if (ret != NULL) {
            return ret;
        }
    }

    return NULL;
}

TreeNode* getTargetCopy(TreeNode* original, TreeNode* cloned, TreeNode* target) {
    return findNode(cloned, target);
}

int main(int argc, char** argv) {
    TreeNode *three = new TreeNode(3, new TreeNode(6), new TreeNode(19));
    TreeNode *root = new TreeNode(7, new TreeNode(4), three);

    TreeNode *node = getTargetCopy(root, root, three);

    return 0;
}