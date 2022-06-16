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
$ cargo run                                                    ✔ 
   Compiling variables v0.1.0 (/home/peng/rust-lang/projects/variables)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:5:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
...
5 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables` due to previous error
```

此示例显示了编译器如何帮助发现程序中的错误。编译器错误可能令人沮丧，但这些编译器错误真的意味着，程序未有安全地执行要程序做的事情；编译器错误并不表示你不是一名良好的程序员！即使那些经验丰富的 Rust 公民，也会收到编译器错误。

该错误消息表示错误原因为 `cannot assing twice to immutable variable 'x'`，是因为有尝试将第二个值赋给那个不可变的 `x` 变量。

在尝试修改某个被指定为不可变的值时，由于这种情况会导致程序错误，因此这个时候收到编译时错误尤为重要。代码一部分的运作，是建立在值将绝不会改变这种假定上，而代码另一部分却修改了那个值，那么就有可能代码的第一部分未有完成他预计要完成的工作了。此类程序错误的原因，就难于追踪到真相，尤其是在代码第二部分仅 *有的时候* 才对那个值进行修改时。Rust 编译器保证了在表明某个值不会变化时，那么那个值就真的不会变化，如此就不必亲自去紧盯着那个变量了。代码也由此而更易于推演。

然而可变则可能会非常有用，并能令到代码更便于编写。变量仅在默认情况下是不可变的；就如同在第 2 章中所做的那样，可通过在变量名字前添加 `mut` 关键字，让变量成为可变。`mut` 的添加，也向将来代码的读者传达了某种意图，表示代码的其他部分，会对这个变量的值进行修改。

比如，来将 `src/main.rs` 修改为下面这样：

文件名：`src/main.rs`

```rust
fn main() {
    let mut x = 5;
    println! ("x 的值为：{}", x);

    x = 6;
    println! ("x 的值为：{}", x);
}
```

在此时运行这个程序时，就会得到这样的输出：

```rust
$ cargo run                                                       101 ✘ 
   Compiling variables v0.1.0 (/home/peng/rust-lang/projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.46s
     Running `target/debug/variables`
x 的值为：5
x 的值为：6
```

在使用了 `mut` 关键字时，就被允许将绑定到 `x` 的值从 `5` 修改到 `6` 了。除了防止程序错误之外，还要考虑多种权衡。比如，在使用着大型数据结构时，就地修改其的一个实例，就会比拷贝并返回新近分配的实例要快一些（for example, in cases where you're using large data structures, mutating an instance in place may be faster than copying and returning newly allocated instances）。而对于较小的数据结构，创建其新实例，并以更具函数式编程风格来编写代码，则可能更易于理解，那么由此带来的性能下降，相对所取得的思路清晰，也会是值得的。

## 常量

与不可变变量类似， *常量（constants）* 是一些绑定到名字且不允许修改的值，但常量与变量之间，有些差异。


