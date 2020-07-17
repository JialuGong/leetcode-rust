class Solution {
public:
	int maximumGap(vector<int>& nums) {
		int size = nums.size();
		if (size == 0 || size == 1) return 0;
		quicksort(nums,0,size-1);
		int i, maxgap;
		maxgap = 0;
		for (i = 1; i < size; i++) {
			maxgap = max(maxgap, nums[i] - nums[i - 1]);
		}
		return maxgap;
	}
	void quicksort(vector<int>&nums, int left, int right ) {
		if (left < right) {
			int pivotpos = partion(nums, left, right);
			quicksort(nums, left, pivotpos - 1);
			quicksort(nums, pivotpos + 1, right);
		}
	}
	int partion(vector<int>&nums, int low, int high) {
		int pivotpos = low;
		int pivot = nums[low];
		for (int i = low + 1; i <= high; i++) {
			if (nums[i] < pivot) {
				pivotpos++;
				if (pivotpos != i) swap(nums[pivotpos], nums[i]);
			}
		}
		nums[low] = nums[pivotpos];
		nums[pivotpos] = pivot;
		return pivotpos;
	}
	int max(int a, int b) {
		if (a > b) return a;
		else return b;
	}
	void swap(int &a, int &b) {
		int c = a;
		a = b;
		b = c;
	}
};
