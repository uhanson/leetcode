struct Node {
    int val;
    int numChildren;
    struct Node** children;
};

int countNodes(struct Node* node, int** columnSizes, int level) {
    if (node->numChildren > 0) {
        *columnSizes[level] += node->numChildren;

        for (int i = 0; i < node->numChildren; i++) {
            countNodes(node->children[i], columnSizes, level + 1);
        }

        return level + 1;
    }

    return level;
};

int** allocColumns(int columnsSize, int **columnSizes) {
    int ** columns = (int **) malloc(sizeof(int*) * columnsSize);

    for (int i = 0; i < columnsSize; i++) {
        columns[i] = (int *) malloc(sizeof(int) * *columnSizes[i]);
    }
}

int fillNodes(struct Node* node, int** columnSizes, int** columns, int level) {
    if (node->numChildren > 0) {
        *columnSizes[level] += node->numChildren;

        int fillPos = 0;
        for (int i = 0; i < node->numChildren; i++) {
            fillNodes(node->children[i], columnSizes, level + 1);
        }
    }
}

/**
 * Return an array of arrays of size *returnSize.
 * The sizes of the arrays are returned as *returnColumnSizes array.
 * Note: Both returned array and *columnSizes array must be malloced, assume caller calls free().
 */
int** levelOrder(struct Node* root, int* returnSize, int** returnColumnSizes) {
    int **returnColumns = (int **) malloc(sizeof(int *) * 1000);

    returnColumnSizes = (int **) malloc(sizeof(int *) * 1000);

    *returnColumnSizes[0] = 1;

    countNodes(root, returnColumnSizes, 1);
}