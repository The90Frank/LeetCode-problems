impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let mut counter : i32 = 0;

        for i in 1..rating.len()-1 {
            let mut leftcounter = 0;
            let mut rightcounter = 0;
            
            for j in 0..i {
                if rating[j] < rating[i] {
                    leftcounter += 1;
                }
            }

            for j in i+1..rating.len() {
                if rating[j] > rating[i] {
                    rightcounter += 1;
                }
            }

            counter += (leftcounter*rightcounter);
        }
        
        for i in 1..rating.len()-1 {
            let mut leftcounter = 0;
            let mut rightcounter = 0;
            
            for j in 0..i {
                if rating[j] > rating[i] {
                    leftcounter += 1;
                }
            }

            for j in i+1..rating.len() {
                if rating[j] < rating[i] {
                    rightcounter += 1;
                }
            }

            counter += (leftcounter*rightcounter);
        }

        return counter;
    }
}
