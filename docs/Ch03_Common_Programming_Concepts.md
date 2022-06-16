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

首先，是不允许在常量上使用 `mut` 关键字的。常量不光是默认不可变的 -- 他们一直都是不可变的。常量的声明用的是 `const` 关键字，而不是 `let` 关键字，同时值的类型 *必须* 被注解（be annotated）。在下一小节，[数据类型](#data-types)，就会讲到类型和类型注解了，因此现在不要关心细节。只要明白必须始终对类型进行注解。

可在任何作用域，包括全局作用域中声明常量。而当在全局作用域中声明常量时，则会让那些代码中许多部分都需要知悉的值的常量，变得有用起来。

常量与不可变变量的最后一个区别，就是常量只能被设置到一个常量表达式，而不能被设置为只能在运行时计算出结果的值。

下面是一个常量声明的示例：

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

该常量的名字为 `THREE_HOURS_IN_SECONDS`，而他的值就被设置为了 `60` （即一分钟的秒数）乘以 `60` （即一小时的分钟数）乘以 `3` （此程序中要计数的小时数）。Rust 关于常量命名的约定，即为全部使用大写，在词汇之间用下划线隔开。编译器在运行时，能够执行一套受限的运算，这样就可以选择将常量值，以这种更易于理解和验证的方式写出来，而不是将该常量设置为值 `10,800`。请参阅 [Rust 参考手册有关常量求值的小节](https://doc.rust-lang.org/reference/const_eval.html)，了解更多有关在声明常量时可使用那些运算的信息。

常量在程序运行的全部时间、在其被声明的作用域内部，都是有效的。常量的这个属性，令到常量对于应用域内的那些、程序多个部分都需要知悉的值来说，变得有用起来，比如某个游戏全部玩家所允许赚到的最大点数，或光速常量。

对那些整个程序都要用到的、作为常量的硬编码值进行取名，对于向代码将来的维护者们传达那些值的意义，是相当有用的。对于未来需要更新硬编码值来说，对常量命名就让那些需要修改的代码只有一处要改，而对此带来帮助。

## 遮蔽（shadowing）

如同在第 2 章中的猜数游戏里看到的那样，可声明一个与先前变量同名的新变量。Rust 公民们表示，那第一个变量是被第二个给 *遮蔽* 掉了，这就意味着在用到这个变量是，程序所看到的，会是第二个变量的值。通过使用一样的变量名字，以及重复使用 `let` 关键字，就可对某个变量进行遮蔽，如下所示：

文件名：`src/main.rs`

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println! ("内部作用域中 x 的值为：{}", x);
    }

    println! ("x 的值为：{}", x);
}
```

```console
内部作用域中 x 的值为：12
x 的值为：6
```

> 注意：遮蔽特性的使用，不需要 `mut` 关键字。

这个程序首先将 `x` 绑定到值 `5`。随后通过重复 `let x =`，取原来的值并加上 `1`，而对 `x` 做了遮蔽操作，因此 `x` 的值此时就为 `6` 了。之后，在一个内部作用域内，第三个 `let` 语句也对 `x` 进行了遮蔽，将先前的值乘以 `2`，就给到 `x` 一个值 `12`。在那个内部作用域完毕时，那个内部遮蔽就结束了，同时 `x` 回到仍为 `6`。在运行这个程序时，他将输出下面的内容：


```console
$ cargo run                                                        ✔ 
   Compiling variables v0.1.0 (/home/peng/rust-lang/projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.47s
     Running `target/debug/variables`
内部作用域中 x 的值为：12
x 的值为：6
```

由于在不小心而尝试在不带 `let` 关键字而重新赋值给该变量时，会收到编译时错误，因此遮蔽不同于构造一个`mut` 的变量。通过使用 `let` 关键字，就可以在值上执行少量的转换操作，而在这些转换操作完成后又将该变量置入到不可变。

`mut` 与遮蔽的另一不同之处，则是由于再次使用`let`关键字时，有效地创建出了一个新变量，因此就可以改变那个值的类型，而仍然重用那同样的变量名字。比如说程序要通过用户输入若干空格字符，来询问用户希望在一些文本之间留多少个空格，而此时又要将用户输入的若干个空格，保存为一个数字：

```rust
let spaces = "    ";
let spaces = spaces.len();
```

第一个 `spaces` 变量是字符串类型，而第二个 `spaces` 变量则是数字类型。遮蔽因此而免于不得不苦苦思索不同的变量名字，诸如 `spaces_str` 及 `spaces_num`；相反，是可以重新较简单的 `spaces` 名称。然而，若尝试对这个变量使用 `mut` 关键字，就会收到一个编译时错误，如下所示：

```rust
let mut spaces = "    ";
spaces = spaces.len();
```

错误是说不允许转变变量类型：

```console
$ cargo run                                                        ✔ 
   Compiling variables v0.1.0 (/home/peng/rust-lang/projects/variables)
error[E0308]: mismatched types
  --> src/main.rs:14:14
   |
13 |     let mut spaces = "    ";
   |                      ------ expected due to this value
14 |     spaces = spaces.len();
   |              ^^^^^^^^^^^^ expected `&str`, found `usize`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `variables` due to previous error
```

现在已经完成变量运行机制的探讨，接卸来就要看看这些变量可以有的那些其余数据类型了。


## 数据类型

Rust 的所有值，都属于某种确切的 *数据类型（data type）*，数据类型告诉 Rust 所指定的是何种数据，进而 Rust 才知道该怎样使用那个数据。接下来会看看两个数据类型的子集：标量（scalar）类型与复合（compound）类型。

请牢记 Rust 是门 *静态类型（statically typed）* 语言，这就意味着在运行时，他必须清楚所有变量的类型。基于值与对变量的使用方式，编译器通常可以推断出希望变量使用何种类型来。在可能有许多中类型的情况下，就如同第 2 章 [将猜数与秘密数字比较](Ch02_Programming_a_Guessing_Game.md#compring-the-guess-to-the-secret-number) 小节中，使用 `parse` 把一个 `String` 转换成数字类型时，就必须添加一个类型注释，如下面这样：

```rust
let guess: u32 = "42".parse().expect("这不是个数字！");
```

若这里添加类型注解，那么 Rust 就会给出下面的错误，表示编译器需要更多信息来明白这里想要使用何种类型：

```console
$ cargo build                                                  101 ✘ 
   Compiling variables v0.1.0 (/home/peng/rust-lang/projects/variables)
error[E0282]: type annotations needed
  --> src/main.rs:19:9
   |
19 |     let guess = "42".parse().expect("那不是个数字！");
   |         ^^^^^ consider giving `guess` a type

For more information about this error, try `rustc --explain E0282`.
error: could not compile `variables` due to previous error
```

接下来就会见识到其他数据类型的类型注解。


## 标量类型（Scalar Types）

*标量* 类型，表示单个值。Rust 有着四个主要的标量类型：整数、浮点数、布尔值与字符。这些类型，其他语言也有。下面就深入看看他们在 Rust 中是怎样工作的。

### 整形（Integer Types）

*整数* 是不带小数部分的数。在第 2 章中就已用到一种整数类型，即 `u32` 类型。这种类型声明表示变量关联的值，应是个无符号的、占据 32 个二进制位空间的整数（有符号整数以 `i` 而不是 `u` 开头）。下面的表 3-1 给出了 Rust 中内建的那些整数类型。可使用这些变种中的任何一个，取声明出某个整数值的类型。

*表 3-1：Rust 中的整数类型*


| 长度 | 有符号 | 无符号 |
| :-: | :- | :- |
| 8 位 | `i8` | `u8` |
| 16 位 | `i16` | `u16` |
| 32 位 | `i32` | `u32` |
| 64 位 | `i64` | `u64` |
| 128 位 | `i128` | `u128` |
| 架构决定 | `isize` | `usize` |

这每个变种，都可以是有符号或无符号的，同时有着显式的大小（二进制位数）。 *有符号* 与 *无符号* 是该数字是否可以是负数--也就是说，该数是否需带有符号（即有符号的），或者说他是否将只为正数，而因此仅能被不带符号的表示出来（即无符号）。这与在纸上写数字相像：在符号重要时，那么写下来的数字就会带有加号或减号；不过在可安全地假定数字为正数时，写下的数字就不带符号了。有符号数字，是采用 [二进制补码](https://en.wikipedia.org/wiki/Two%27s_complement) 表示法加以存储的。

每个有符号变种，都可存储自 `-(2^n-1)` 到 `2^n-1` 范围的那些数字（包括边界上的两个数字），其中的 `n` 即为变种用到的位数。

