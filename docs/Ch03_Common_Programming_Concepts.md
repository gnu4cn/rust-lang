# 常见编程概念

本章涵盖了出现在几乎所有编程语言中的一些概念，以及这些概念在 Rust 中运作方式。众多编程语言，在他们各自核心，都有着许多共同的东西。出现在本章中的这些概念，没有一个是 Rust 独有的，然而这里是要就他们在 Rust 语境下进行讨论，并对使用这些概念的相关约定进行解释。

具体来讲，本章将要掌握到变量、基本类型、函数、注释及控制流等概念。这些基本概念将出现在全部 Rust 程序中，而早点掌握他们，就会给到一个强大的核心起点。

> **关键字（keywords）**
>
> Rust 语言有着一套 *关键字*，他们是保留的，仅由语言使用，在这点上与其他语言没有什么不同。牢记是不可以将这些词汇，用作变量或函数的名称的。多数关键字都有着特殊意义，而会使用他们来完成 Rust 程序中不同任务；其中有少数几个当前并没有关联功能，他们被保留用于未来将添加到 Rust 的功能。在 [附录 A](Ch99_Appendix_A.md) 就能看到关键字清单。


## 变量与可变性

**Variables and Mutability**

就如在之前的 ["用变量保存值"](Ch02_Programming_a_Guessing_Game.md#storing-values-with-variables) 小节中所讲的那样，默认变量是不可变的。这是 Rust 所提供的，推动利用 Rust 赋予的安全和易于并发代码编写方式的众多措施之一（by default variables are immutable, this is one of many nudges Rust gives you to write your code in a way that takes advantage of the safety and easy concurrency that Rust offers）。尽管如此，还是有将变量作为可变的选项。下面就来搞清楚，为何 Rust 会鼓励偏向不可变，以及为何有时会希望不接受 Rust 的建议。

在变量为不可变时，一旦值被绑定到某个名字，那么就无法修改那个值了。为对此进行演示，就来通过使用 `cargo new variables` 在 `projects` 目录中生成一个新的名为 `variables` 的项目。

然后，在那个新的 `variables` 目录中，打开 `src/main.rs` 并将其代码替换为下面的代码。此代码当然不会被编译，这里首先要对不可变错误加以检视。

```rust
fn main() {
    let x = 5;
    println! ("x 的值为：{}", x);

    x = 6;
    println! ("x 的值为：{}", x);
}
```

保存并使用 `cargo run` 运行这个程序。就会受到错误消息，如下面这个输出：

```console

