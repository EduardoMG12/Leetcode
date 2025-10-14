# LeetCode Solutions in Rust 🦀

A collection of LeetCode problem solutions implemented in Rust, focusing on efficient algorithms and clean code practices.

## 📊 Progress Overview

- **Total Problems Solved:** 3
- **Language:** Rust
- **Difficulty Levels:** Easy to Medium

## 🗂️ Problem Solutions

### Easy Problems

<details>
<summary><strong>1. Two Sum</strong> - <em>Array, Hash Table</em></summary>

**Problem:** Given an array of integers `nums` and an integer `target`, return indices of the two numbers such that they add up to `target`.

**File:** [`rust/two_sum/main.rs`](rust/two_sum/main.rs)

**Approach:** Hash Table
- **Time Complexity:** O(n)
- **Space Complexity:** O(n)

**Key Points:**
- Uses HashMap to store previously seen numbers and their indices
- Single pass algorithm for optimal time complexity
- Handles the constraint that each input has exactly one solution

**Example:**
```rust
Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: nums[0] + nums[1] = 2 + 7 = 9
```

</details>

<details>
<summary><strong>20. Valid Parentheses</strong> - <em>String, Stack</em></summary>

**Problem:** Given a string `s` containing just the characters `'('`, `')'`, `'{'`, `'}'`, `'['` and `']'`, determine if the input string is valid.

**Files:** 
- [`rust/valid_parentheses/main.rs`](rust/valid_parentheses/main.rs) - HashMap approach
- [`rust/valid_parentheses/better_solution.rs`](rust/valid_parentheses/better_solution.rs) - Stack approach (optimized)

**Approaches:**

1. **HashMap Approach** (Initial Solution)
   - **Time Complexity:** O(n)
   - **Space Complexity:** O(n)
   - Uses HashMap to track opening brackets and their positions

2. **Stack Approach** (Better Solution)
   - **Time Complexity:** O(n)
   - **Space Complexity:** O(n)
   - Uses Vec as stack for cleaner and more intuitive implementation
   - More memory efficient in practice

**Key Points:**
- Stack-based solution is more conventional and readable
- Handles all three types of brackets: (), [], {}
- Validates proper nesting and matching

**Example:**
```rust
Input: s = "()[]{}"
Output: true

Input: s = "([)]"
Output: false
```

</details>

### Medium Problems

<details>
<summary><strong>21. Merge Two Sorted Lists</strong> - <em>Linked List, Recursion</em></summary>

**Problem:** You are given the heads of two sorted linked lists `list1` and `list2`. Merge the two lists in a sorted manner.

**File:** [`merge_two_sorted_lists/main.rs`](merge_two_sorted_lists/main.rs)

**Status:** 🚧 Work in Progress

**Current Implementation:**
- Custom ListNode structure defined
- Working on merge and sorting algorithm
- Exploring bubble sort approach for list merging

**Planned Approach:**
- **Time Complexity:** O(m + n) where m and n are lengths of the lists
- **Space Complexity:** O(1) for iterative approach

**Key Points:**
- Two-pointer technique for efficient merging
- Handles edge cases (empty lists)
- Maintains sorted order throughout the merge process

</details>

## 🛠️ How to Run

### Prerequisites
- Rust installed on your system
- Cargo package manager

### Running Solutions

1. **Navigate to the specific problem directory:**
   ```bash
   cd rust/two_sum
   # or
   cd rust/valid_parentheses
   # or
   cd merge_two_sorted_lists
   # or 
   ...
   ```

2. **Compile and run:**
   ```bash
   rustc main.rs -o main
   ./main
   ```

   Or for better solutions:
   ```bash
   rustc better_solution.rs -o better_solution
   ./better_solution
   ```

3. **Using Cargo (if Cargo.toml exists):**
   ```bash
   cargo run
   ```

## 📁 Project Structure

```
leetcode/
├── rust/
│   ├── two_sum/
│   │   ├── main.rs          # Hash table solution
│   │   └── main             # Compiled binary
│   └── valid_parentheses/
│       ├── main.rs          # HashMap approach
│       ├── main             # Compiled binary
│       ├── better_solution.rs # Stack approach (recommended)
│       └── better_solution    # Compiled binary
├── merge_two_sorted_lists/
│   ├── main.rs              # Work in progress
│   └── main                 # Compiled binary
└── README.md
```

## 🎯 Solution Strategies

<details>
<summary><strong>Hash Table / HashMap</strong></summary>

Used in problems requiring fast lookups and mappings:
- **Two Sum:** Maps values to indices for O(1) complement lookup
- **Valid Parentheses (v1):** Maps positions to opening brackets

**Pros:**
- O(1) average lookup time
- Flexible key-value storage

**Cons:**
- Higher memory usage
- Hash collision potential

</details>

<details>
<summary><strong>Stack (Vec in Rust)</strong></summary>

Perfect for problems with nested structures or LIFO requirements:
- **Valid Parentheses:** Tracks opening brackets for proper matching

**Pros:**
- Simple LIFO operations
- Memory efficient for this use case
- Intuitive for bracket matching

**Cons:**
- Limited to stack operations
- Not suitable for random access

</details>

<details>
<summary><strong>Two Pointers</strong></summary>

Efficient for array/list traversal problems:
- **Merge Two Sorted Lists:** Compare elements from both lists

**Pros:**
- Space efficient O(1) extra space
- Often reduces time complexity

**Cons:**
- Requires sorted or structured data
- Can be complex to implement correctly

</details>

## 🚀 Future Improvements

- [ ] Complete the Merge Two Sorted Lists implementation
- [ ] Add more comprehensive test cases
- [ ] Implement alternative solutions for existing problems
- [ ] Add benchmark comparisons between different approaches
- [ ] Create Cargo workspace for better dependency management
- [ ] Add documentation with rustdoc
- [ ] Implement CI/CD for automated testing

## 📚 Learning Resources

- [LeetCode Platform](https://leetcode.com/)
- [Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust Algorithms Repository](https://github.com/EbTech/rust-algorithms)

## 🤝 Contributing

Feel free to suggest improvements, alternative solutions, or point out any issues in the implementations!

---

*Happy Coding! 🦀*
