struct Solution;

impl Solution {
    pub fn min_number_of_hours(initial_energy: i32, mut initial_experience: i32, energy: Vec<i32>, experience: Vec<i32>) -> i32 {
        let mut ans = 0;

        let sum: i32 = energy.iter().sum();
        if initial_energy < sum + 1 {
            ans += sum + 1 - initial_energy;
        }

        for i in 0..experience.len() {
            if initial_experience < experience[i] + 1 {
                ans += experience[i] + 1 - initial_experience;
                initial_experience = experience[i] + 1;
            }

            initial_experience += experience[i];
        }

        ans
    }
}