// `pub` 意思是 "public"（公共的），类似于 JavaScript 中的 `export`
// 它使得 `Solution` 可以在测试用例中被访问
pub struct Solution;

// 本地模拟 LeeCode 的代码结构，减少编辑器切换的麻烦，直接复用
// `impl Solution` 就是 `Solution` 结构体的实现（implementing）
// 这里的 `Solution` 是一个结构体，类似于 JS 中的对象
// 可以在结构体中加个函数什么的
impl Solution {
    // 在结构体中定义一个公共函数
    //
    // 参数:
    // `nums: Vec<i32>`:
    //   - `nums` 是参数名。
    //   - `Vec<i32>` 是类型。`Vec` 是一个 "Vector"，Rust 版本的可增长数组（类似 JS 数组）。
    //   - `<i32>` 表示这个 vector 包含的是 32 位有符号整数 (例如: -10, 0, 5, 100)。
    // `target: i32`:
    //   - `target` 是另一个参数，也是一个 32 位整数。
    //
    // 返回类型 (Return type):
    // `-> Vec<i32>`: 这个箭头表示函数的返回类型。它将返回一个 `i32` 类型的 Vector。
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 这就像一个 `import` 语句。它将 `HashMap` 引入到当前作用域。
        // `HashMap` 类似于 JavaScript 的 `Map` 对象，或者使用一个普通对象 `{}` 作为键值存储。
        // `std` 是 Rust 的标准库。
        use std::collections::HashMap;
        // `HashMap::new()`: 创建一个新的、空的 HashMap
        // 就像 `const map = new Map();` 或 `const map = {};`。
        let mut map = HashMap::new();
        // 这是一个 `for` 循环，用于遍历 `nums` vector。
        // `nums.iter()`: 为 vector 创建一个“迭代器（iterator）”。可以把它看作是遍历元素的辅助工具。
        // `.enumerate()`: 迭代器上的一个方法，它会同时提供每个元素的索引和值。
        //                 类似于 `nums.forEach((num, i) => ...)` 或者 `for (let i = 0; i < nums.length; i++)`。
        // `(i, &num)`: 这是“解构（destructuring）”。`i` 获取索引（一个整数），`num` 获取元素。
        //              `num` 前面的 `&` 表示我们得到的是对数字的“引用（reference）”，不是拷贝。
        //              这里的 & 在模式中起到了“解引用”或者说“剥离引用层”的作用。
        //              它表示“我期望这里是一个引用，请把这个引用指向的值绑定给变量 num”
        //              所以在循环体中，我们可以直接使用 `num` 而不需要额外的解引用操作。
        for (i, &num) in nums.iter().enumerate() {
            // `target - num` 计算出我们需要的另一个数字。
            // 例如
            //      nums = vec![2, 7, 11, 15];
            //      target = 9;
            //      target-num = 9 - 2 = 7;
            // `map.get(&(target - num))`: 尝试在 `map` 中查找键为 `(target - num)` 的项。
            //   - 在 Rust 中，`map.get()` 不会像 JS 那样直接返回值或者 `null/undefined`。
            //   - 它返回一个 `Option` 类型，如果找到了就是 `Some(value)`，没找到就是 `None`。
            //     这有助于防止因值缺失而产生的错误。
            // `if let Some(&j) = ...`: 这是 Rust 中一个常见的模式。
            //   - 它检查 `map.get(...)` 是否返回了 `Some(value)`。
            //   - 如果是，它会“解开”这个值并赋给 `j`。
            //   - 所以，如果 `target - num` 是 `map` 中的一个键，`j` 就会是它对应的值（一个索引）。
            //   - 这是一种比 `if (map.has(key)) { const j = map.get(key); ... }` 更安全的方式。
            if let Some(&j) = map.get(&(target - num)) {
                // 生成一个新的数组，存储两个数值的下标（这两个下标对应的值相加就是 `target`）
                // return 会结束循环
                return vec![j as i32, i as i32];
            }
            // 在每次循环迭代中，迭代器会消耗一个元素。如果当前元素没有在 HashMap 中找到它的配对数，
            // 那么当前元素及其索引就会被添加到 HashMap 中，使其条目增加一个。
            // 第一次循环时，`map` 为空，所以 `map.get(...)` 不会找到任何值。
            // 第二次循环时，`map` 可能会有一个值，但如果当前的 `num` 没有找到配对，
            // 那么当前的 `num` 和它的索引 `i` 就会被添加到 `map` 中。
            map.insert(num, i);
        }
        // 如果循环结束都没有找到一对数，就返回一个空的 vector。
        // 在 Rust 中，函数中最后一个没有分号的表达式会作为隐式返回值。
        // 所以，这就像 `return [];`。
        vec![]
    }
}

fn main() {
}

// 表示 `mod tests` 这个代码块只在运行测试的时候 (例如 `cargo test`) 才会被编译和包含。
#[cfg(test)]
mod tests {
    // `use super::*;` 从父模块 (`super`) 导入所有内容 (`*`)。
    // 在这个例子中，它使得 `Solution` (以及 `two_sum` 方法) 在 `tests` 模块内部可用。
    // 如果这是一个单独的测试文件，可能类似于 `import { Solution } from '../';`。
    use super::*;

    // 这个属性将函数 `test_two_sum` 标记为一个测试函数。
    // 测试运行器 (通过 `cargo test` 调用) 会找到并执行带有这个属性的函数。
    #[test]
    fn test_two_sum() {
        println!("Running Problem 1001: Two Sum");
        let nums1 = vec![2, 7, 11, 15];
        let target1 = 9;
        let result1 = Solution::two_sum(nums1.clone(), target1);
        println!("Input: nums = {:?}, target = {}", nums1, target1);
        println!("Output: {:?}", result1);

        let nums2 = vec![3, 2, 4];
        let target2 = 6;
        let result2 = Solution::two_sum(nums2.clone(), target2);
        println!("Input: nums = {:?}, target = {}", nums2, target2);
        println!("Output: {:?}", result2);

        let nums3 = vec![3, 3];
        let target3 = 6;
        let result3 = Solution::two_sum(nums3.clone(), target3);
        println!("Input: nums = {:?}, target = {}", nums3, target3);
        println!("Output: {:?}", result3);
    }
}

/*

Rust 的模式匹配与前端开发类比：

const nums = [10, 20, 30];
nums.forEach((value, index) => {
    // 这里 value 直接是值，index 是索引
    // Rust 的 (i, &num) 模式更强大，因为它能处理引用并直接绑定到值
});

*/