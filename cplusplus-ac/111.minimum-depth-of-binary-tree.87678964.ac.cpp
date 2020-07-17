/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class Solution {
public:
    int minDepth(TreeNode* ptr) {
      if(ptr==NULL) return 0;
       else if(ptr->left==NULL) return 1+minDepth(ptr->right);
       else if(ptr->right==NULL) return 1+minDepth(ptr->left);
       else{
        int leftdepth=minDepth(ptr->left);
        int rightdepth=minDepth(ptr->right);
        return 1+min(rightdepth,leftdepth);
       } 
    }
};
