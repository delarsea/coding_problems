/*
standard twosum algorithm question:

Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.
*/

#include <iostream>     // std::cout
#include <algorithm>    // std::sort
#include <vector>       // std::vector
using namespace std; 

class myexception: public exception
{
  virtual const char* what() const throw()
  {
    return "No solution in input vector";
  }
} myex;

class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        std::sort(nums.begin(), nums.end());
        int start = 0;
        int end = nums.size() - 1;
        while (start < end){
            if (nums[start] + nums[end] == target)
                return vector <int> {start, end};
            else if (nums[start] + nums[end] < target)
                start++;
            else
                end --;
        };
        throw myex;
    };
};