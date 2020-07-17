class Solution {
public:
    double findMedianSortedArrays(vector<int>& nums1, vector<int>& nums2) {
      	int size1, size2,i,j,k;
		double ans;
		size1 = nums1.size();
		size2 = nums2.size();
		if (size1 == 0) {
			if (size2 % 2 == 0) {
				i = size2 / 2 - 1;
				j = size2 / 2;
				ans = (double(nums2[i] )+ double(nums2[j])) / 2;
			}
			else {
				i = size2 / 2;
				ans = double(nums2[i]);
			}
		}
		if(size2==0) {
			if (size1 % 2 == 0) {
				j = size1/ 2 - 1;
				j = size1 / 2;
				ans =(double(nums1[i]) + double(nums1[j]) )/ 2;
			}
			else {
				i = size1 / 2;
				ans = double(nums1[i]);
			}
		}
		if (size1 != 0 && size2 != 0) {
			vector<int> v;
			for (i = 0, k = 0, j = 0; k < (size1 + size2 + 2) / 2; ) {
				if (nums1[i] > nums2[j]) v.push_back(nums2[j]), j++;
				else v.push_back(nums1[i]), i++;
				k++;
				if (j == size2) {
					for (; k < (size1 + size2 + 2) / 2; k++) {
						v.push_back(nums1[i]);
						i++;
					}
				}
				if (i == size1) {
					for (; k < (size1 + size2+2) / 2; k++) {
						v.push_back(nums2[j]);
						j++;
					}
				}
			}
			if ((size1 + size2) % 2 == 0) ans = (double(v[k - 1] )+double(v[k - 2]))/ 2;
			else ans = double(v[k - 1]);
		}
		return ans;
	}
        
};
