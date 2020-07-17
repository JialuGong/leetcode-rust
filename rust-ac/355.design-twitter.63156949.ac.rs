use std::collections::HashMap;
struct Twitter {
    user_db: Vec<User>,
    count: usize,
}
struct User {
    user_id: i32,
    follow: Vec<i32>,
    tweet: Vec<(i32, usize)>,
}
impl User {
    pub fn add_tweet(&mut self, tweet_id: i32, count: usize) {
        self.tweet.push((tweet_id, count));
    }
    pub fn add_follow(&mut self, user_id: i32) {
         let mut index = -1;
        for (i, item) in self.follow.iter().enumerate() {
            if item == &user_id {
                index = i as i32;
                break;
            }
        }
        if index==-1{
            self.follow.push(user_id);
        }
        
    }
    pub fn un_follow(&mut self, user_id: i32) {
        let mut index = -1;
        for (i, item) in self.follow.iter().enumerate() {
            if item == &user_id {
                index = i as i32;
                break;
            }
        }
        if index != -1 {
            self.follow.remove(index as usize);
        }
    }
    pub fn query_tweet(&self)->Vec<(i32,usize)>{
        let mut ans =Vec::new();
        for item in &self.tweet{
            ans.push((item.0,item.1));
        }
        ans
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Twitter {
            user_db: Vec::new(),
            count: 0,
        }
    }

    /** Compose a new tweet. */
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.count += 1;
        let mut flag = false;
        for item in &mut self.user_db {
            if item.user_id == user_id {
                flag = true;
                item.add_tweet(tweet_id,self.count);
                break;
            }
        }
        if !flag {
            self.user_db.push(User {
                user_id: user_id,
                tweet: vec![(tweet_id,self.count)],
                follow: vec![],
            });
        }
    }

    /** Retrieve the 10 most recent tweet ids in the user's news feed. Each item in the news feed must be posted by users who the user followed or by the user herself. Tweets must be ordered from most recent to least recent. */
    fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        let mut tweets:Vec<(i32,usize)>=Vec::new();
        for item in & self.user_db {
            if item.user_id == user_id {
                tweets.append(&mut item.query_tweet());
                let follow=&item.follow;
                for follow_id in follow{
                    for fo in &self.user_db{
                        if fo.user_id==*follow_id{
                            tweets.append(&mut fo.query_tweet());
                        }
                    }
                }

                tweets.sort_by(|a,b| b.1.cmp(&a.1));
                let mut ans=Vec::new();
                for i in 0..10{
                    if tweets.get(i).is_some(){
                        ans.push(tweets.get(i).unwrap().0)
                    }else {
                        break;
                    }
                }
               return  ans;
            }
        }
        Vec::new()
    }

    /** Follower follows a followee. If the operation is invalid, it should be a no-op. */
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        if follower_id!=followee_id{
              let mut flag=false;
        for item in &mut self.user_db {
            if item.user_id == follower_id {
                item.add_follow(followee_id);
                flag=true;
                break;
            }
        }
        if !flag{
            self.user_db.push(User{user_id:follower_id,follow:vec![followee_id],tweet:vec![]});
        }
        }
      
    }

    /** Follower unfollows a followee. If the operation is invalid, it should be a no-op. */
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        for item in &mut self.user_db {
            if item.user_id == follower_id {
                item.un_follow(followee_id);
            }
        }
    }
}
