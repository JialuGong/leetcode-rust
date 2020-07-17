class Solution {
public:
	vector<int>intersection(vector<int>& nums1, vector<int>& nums2) {
		int size1, size2;
		vector<int>ans;
		size1 = nums1.size();
		size2 = nums2.size();
        if(size1==0||size2==0) return ans;
		quicksort(nums1, 0, size1 - 1);
		quicksort(nums2, 0, size2 - 1);
		for (int i = size1-1; i>=0;i--){
			if (BiSearch(nums1[i],nums2)) ans.push_back(nums1[i]);
		}
		return ans;
	}
	bool BiSearch(int num, vector<int>& nums) {
		int size = nums.size();
		int i;
		bool ans = binarysearch(num, nums, 0, size-1);
		for (i = size - 1; i >= 0; i--) {
			if (nums[i]< num) break;
            nums.pop_back();
		}
		return ans;
	}
	bool binarysearch(int num, vector<int>& nums, int left, int right) {
		if (left > right) return false;
		int mid = (right + left) / 2;
		if (num == nums[mid]) return true;
		else if(num<nums[mid])
		return binarysearch(num, nums, left, mid - 1);
		else 
		return binarysearch(num, nums, mid + 1, right);
	}
	void quicksort(vector<int>& nums, int left, int right) {
		if (left < right) {
			int pivotpos = partion(nums,left, right);
			quicksort(nums, left, pivotpos - 1);
			quicksort(nums, pivotpos + 1, right);
		}
	}

	int partion(vector<int>&nums, int low, int high) {
		int pivotpos = low;
		int pivot = nums[low];
		int i;
		for (i = low + 1; i <= high; i++) {
			if (nums[i] < pivot) {
				pivotpos++;
				if (pivotpos != i) swap(nums[pivotpos], nums[i]);
			}
		}
		nums[low] = nums[pivotpos];
		nums[pivotpos] = pivot;
		return pivotpos;
	}
};
