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
   int minDiffInBST(TreeNode *root) {
		vector<int>v;
		treetovector(root, v);
		int i,minnum;
		minnum = 10000;
		for (i = 1; i <v.size(); i++) {
			minnum = min(minnum,fabs(v[i] - v[i - 1]));
		}
		return minnum;
	}
	int min(int a, int b) {
		if (a > b) return b;
		else return a;
	}
	int fabs(int a) {
		if (a > 0) return a;
		else return -a;
	}
	void treetovector(TreeNode *root, vector<int>& v) {
		if (root == NULL) return;
		treetovector(root->left,v);
        v.push_back(root->val);
		treetovector(root->right,v);
	}
};
