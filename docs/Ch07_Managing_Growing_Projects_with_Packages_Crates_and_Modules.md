# 用代码包、代码箱与模组来对日益增长的项目进行管理

在编写大型程序时，由于在头脑里对整个程序保持追踪已成为不可能，因此代码组织就尤为重要。通过将相关功能分组，并以截然不同的特性而将代码加以分离，就会搞清楚在哪里去找到实现了某个特定特性的代码，以及在那里去修改某项特性的运作方式。

到目前为止，这里所编写的程序，都是在一个模组的一个文件中的。而随着项目的增长，就可以通过将项目分解为多个模组及多个文件，来对代码加以组织。一个代码包，可以包含多个二进制的代码箱，并可有选择地包含一个库代码箱。本章会涵盖到所有的这些技巧。对于那些极为大型、有着一套互相关联而共同演化的项目，Cargo 工具提供了工作区（workspaces）概念，关于工作区，将在第 14 章的 [Cargo 工作区](Ch14_More_about_Cargo_and_Crates_io.md#cargo-workspaces)中讲到。

除了功能分组（grouping functionality），对功能实现细节的封装，还实现了高级别的代码重用：一旦实现了某个操作，其他代码就可以在无需掌握该实现原理的情况下，通过该代码的公共接口对该实现代码加以调用。编写代码的方式，就定义了哪些部分是公开给其他代码使用的，哪些部分是私有的实现细节而保留着修改的权力。这是另一种限制那些必须保留在头脑中的细节数量的方式（in addition to grouping functionality, encapsulating implementation details lets you reuse code at a higher level: once you've implemented an operation, other code can call that code via the code's pulic interface without knowing how the implementation works. The way you write code defines which part are public for other code to use and which parts are private implementation details that you reserve the right to change. This is another way to limit the amount of detail you have to keep in your head）。

与此相关的概念，就是作用域（scope）：代码编写所在的嵌套语境，有着一套定义在所谓 “在作用域里” 的名字。在代码的读、写及编译时，程序员与编译器都需要掌握，在某个特定点位的某个特定名字，是否是指向了某个变量、函数、结构体、枚举、模组、常量或别的项目，以及该名字所指向项目的意义。作用域的创建，以及改变哪些名字在或不在某个作用域，都是可行的。在同一作用域不能有两个名字相同的项目；是有一些工具，来找出名字冲突的。

Rust 有着数种实现对代码组织进行管理的特性，包括哪些细节被暴露、哪些细节为私有，以及程序中各个作用域中有哪些名字。Rust的这些有关代码组织的特性，有时被统称为 *模组系统（module system）*，包括了：

- **代码包（packages）**：实现代码箱（crates）的构建、测试与分享的 Cargo 特性；
- **代码箱（crates）**：产生出库或可执行文件的模组树（a tree of modules that produces a library or executable）；
- **模组（modules）**与 **`use`关键字**：实现对代码组织、作用域及路径私有化的控制（let you control the organization, scope, and privacy of paths）；
- **路径（paths）**：对结构体、函数或模组等进行命名的方式（a way of naming an item, such as a struct, function, or module）。

在本章中，就要涉及到这些特性，讨论到他们之间互动的原理，并就如何使用这些特性来对作用域进行管理。在本章结束时，就会对 Rust 的模组系统有扎实掌握，并能够像专业 Rust 程序员那样，以作用域来编写程序！

## 代码包与代码箱

Rust 模组系统的第一部分，将涉及到代码包与代码箱。

*代码包（package）* 即为一个或多个的、提供一套功能集的代码箱。代码包包含了一个描述了如何来构建这些代码箱的 `Cargo.toml` 文件。

而 *代码箱（crate）* 则可以是二进制代码箱（a binary crate），或者是库代码箱(a library crate)。*二进制代码箱* 是一些可编译为可以运行的一些程序，譬如命令行的程序或服务器程序。二进制代码箱必须有着一个叫做 `main` 的、定义了在可执行文件运行时所发生事情的函数。到目前为止本书中创建的全部代码箱，都是二进制代码箱。

*库代码箱* 是没有 `main` 函数的，且他们不会编译到可执行文件。他们对计划在多个项目下使用的功能进行定义。比如在 [第二章](Ch02_Programming_a_Guessing_Game.md#generating-a-random-number) 中用到的、提供了生成随机数功能的 `rand` 代码箱。

*代码箱根（crate root）* 是个 Rust 编译器开始之处的源文件，并构成了代码箱的根模组（the *crate root* is a source file that the Rust compiler starts from and makes up the root module of your crate. 后面在 [定义对作用域和私有化进行控制的模组](#defining-modules-to-control-scope-and-privacy) 小节，将深入探讨到模组概念）。

代码包能包含些什么，是由数条规则确定的。一个代码包可包含至多一个的库代码箱。而对于二进制代码箱，则是想要多少就可包含多少，不过代码包必须包含至少一个代码箱（库或二进制均可）。

下面就来过一下在创建代码包时，发生了些什么。首先，这里要敲入命令 `cargo new`:

```console
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project                                                                            ✔ 
Cargo.toml  src
$ ls my-project/src                                                                        ✔ 
main.rs
```

在敲入了该命令后，Cargo 就创建出了 `Cargo.toml` 文件，于是给到了一个代码包。查看 `Cargo.toml` 文件的内容，其中由于 Cargo 依循了一条 `src/main.rs` 即为与该代码包同名二进制代码箱的箱根的约定，因此 `Cargo.toml` 内容中并未提及 `src/main.rs`。与此类似，Cargo 还知道在代码包目录包含了 `src/lib.rs` 时，这个代码包就包含了与该包同名的一个库代码箱，而那个 `src/lib.rs` 就是该库代码箱的箱根。Cargo 会将代码箱根文件，传递给 `rustc` 来构建出相应的库或二进制程序。

这里有一个只包含了 `src/main.rs` 的代码包，就意味着他只包含了名为 `my-project` 的二进制代码箱。而在代码包同时包含了 `src/main.rs` 与 `src/lib.rs` 时，他就会有两个代码箱：一个二进制和一个库代码箱，二者都有着与该代码包同样的名字。代码包通过将一些文件放入到 `src/bin` 目录：每个文件都将是单独的二进制代码箱，这样就能够有着多个二进制代码箱。


## 定义对作用域和私有化进行控制的模组

在本小节中，将讲到模组与模组系统的其他部分，即实现对各种项目（items, 变量、函数、结构体、枚举、模组、常量或别的项目）命名的 *路径（paths）*；还有将某个路径引入到作用域的 `use` 关键字；以及将那些项目构造为公开项目的 `pub` 关键字。这里还将对 `as` 关键字、外部代码包，以及全局操作符等加以讨论。

首先，这里将以一个往后对代码进行组织时，易于参考的规则列表开始。随后就会对这些详细解释。

### 模组速查

下面就是模组、路径、`use` 关键字与 `pub` 关键字在编译器中的工作原理，以及多数开发者组织他们代码的方式。
