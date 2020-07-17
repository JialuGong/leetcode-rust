/**
 * Definition for an interval.
 * struct Interval {
 *     int start;
 *     int end;
 *     Interval() : start(0), end(0) {}
 *     Interval(int s, int e) : start(s), end(e) {}
 * };
 */
class Solution {
public:
	vector<Interval> insert(vector<Interval>& intervals, Interval newInterval) {
		int size = intervals.size();
		vector<Interval>ans;
		if (size == 0) {
			ans.push_back(newInterval);
			return ans;
		}
        if (newInterval.end < intervals[0].start) {
			ans.push_back(newInterval);
			for (int i = 0; i < size; i++)
				ans.push_back(intervals[i]);
            return ans;
		}
		int i,r,l;
		int left = 0;
		int right =size;
		int gap = newInterval.end - newInterval.start;
		bool isin;
		find(0, size - 1, intervals, newInterval.start,true,l,gap,isin);
		if (l == size) {
			ans = intervals;
			ans.push_back(newInterval);
			return ans;
		}
		find(0, size - 1, intervals, newInterval.end,false,r,gap,isin);
		if (r == -1) {
			ans.push_back(newInterval);
			for (i = 0; i < size; i++)
				ans.push_back(intervals[i]);
			return ans;
		}
		for (i = 0; i < l; i++) {
			ans.push_back(intervals[i]);
		}
		if (isin) {
			Interval in(intervals[l].start, intervals[r].end);
			ans.push_back(in);
		}
		else
		{
			ans.push_back(intervals[l]);
			ans.push_back(newInterval);
		}
		for (i = r + 1; i < size; i++) {
			ans.push_back(intervals[i]);
		}
		return ans;
	}
	void find(int left, int right, vector<Interval>& intervals, int val,bool isstart,int &ans,int gap,bool& isin) {
		isin = true;
		int	mid = (right + left) / 2;
		if (val > intervals[mid].end && mid >=int(intervals.size()-1)) {
			if (isstart) { ans = mid + 1; return; }
			else {
				intervals[mid].end = val;
				ans= mid;
				return;
			}
		}
		if (val <intervals[mid].start&&mid == 0) {
			if (isstart) {
				intervals[0].start = val;
				ans=mid;
				return;
			}
			else { ans = -1; return; }
		}
		if (val <intervals[mid].end&&val > intervals[mid].start) {
			ans = mid;
			return;
		}
		if (val == intervals[mid].end || val == intervals[mid].start) {
			ans = mid;
			return;
		}
		if (val > intervals[mid].end&&val < intervals[mid + 1].start) {
			if (isstart) {
				if (gap < (intervals[mid + 1].start - val)) {
					isin = false;
					ans = mid;
					return;
				}
				else {
					intervals[mid + 1].start = val;
					ans = mid + 1;
					return;
				}
			}
			else {
				if (gap < (val - intervals[mid].end)) {
					ans = mid;
					isin = false;
					return;
				}
				else {
					intervals[mid].end = val;
					ans = mid;
					return;
				}
			}
		}
		if (val < intervals[mid].start) find(left, mid - 1, intervals, val, isstart, ans,gap,isin);
		if (val > intervals[mid].end) find(mid + 1, right, intervals, val, isstart ,ans,gap,isin);
	}
};
