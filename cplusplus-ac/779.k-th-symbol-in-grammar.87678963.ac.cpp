class Solution{
private:
   void find_it(vector<int>&nums,int k,int n){
       if(n==1) return ;
       else{
        if(k%2==1) nums.push_back(0);
       else nums.push_back(1);
        k=(k+1)/2;
        find_it(nums,k,--n);
       }
   }
public:
    int kthGrammar(int N, int K){
        vector<int> nums;
        find_it(nums,K,N);
        int num=0;
        int size_=nums.size();
        for(int i=size_-1;i>=0;i--){
            if(num==0){
                if(nums[i]==0) num=0;
                if(nums[i]==1) num=1;
            }
            else{
                if(nums[i]==0) num=1;
                if(nums[i]==1) num=0;
            }
             cout<<nums[i]<<endl;
            cout<<"num:"<<num<<endl;
        }
        cout<<endl;//
        return num;
    }
};
