# 用代码包、代码箱与模组来对日益增长的项目进行管理

在编写大型程序时，由于在头脑里对整个程序保持追踪已成为不可能，因此代码组织就尤为重要。通过将相关功能分组，并以截然不同的特性而将代码加以分离，就会搞清楚在哪里去找到实现了某个特定特性的代码，以及在那里去修改某项特性的运作方式。

到目前为止，这里所编写的程序，都是在一个模组的一个文件中的。而随着项目的增长，就可以通过将项目分解为多个模组及多个文件，来对代码加以组织。一个代码包，可以包含多个二进制的代码箱，并可有选择地包含一个库代码箱。本章会涵盖到所有的这些技巧。对于那些极为大型、有着一套互相关联而共同演化的项目，Cargo 工具提供了工作区（workspaces）概念，关于工作区，将在第 14 章的 [Cargo 工作区](Ch14_More_about_Cargo_and_Crates_io.md#cargo-workspaces)中讲到。

除了功能分组（grouping functionality），对功能实现细节的封装，还实现了高级别的代码重用：一旦实现了某个操作，其他代码就可以在无需掌握该实现原理的情况下，通过该代码的公共接口对该实现代码加以调用。编写代码的方式，就定义了哪些部分是公开给其他代码使用的，哪些部分是私有的实现细节而保留着修改的权力。这是另一种限制那些必须保留在头脑中的细节数量的方式（in addition to grouping functionality, encapsulating implementation details lets you reuse code at a higher level: once you've implemented an operation, other code can call that code via the code's pulic interface without knowing how the implementation works. The way you write code defines which part are public for other code to use and which parts are private implementation details that you reserve the right to change. This is another way to limit the amount of detail you have to keep in your head）。

与此相关的概念，就是作用域（scope）：代码编写所在的嵌套语境，有着一套定义在所谓 “在作用域里” 的名字。在代码的读、写及编译时，程序员与编译器都需要掌握，在某个特定点位的某个特定名字，是否是指向了某个变量、函数、结构体、枚举、模组、常量或别的项目，以及该名字所指向项目的意义。作用域的创建，以及改变哪些名字在或不在某个作用域，都是可行的。在同一作用域不能有两个名字相同的项目；是有一些工具，来找出名字冲突的。

Rust 有着数种实现对代码组织进行管理的特性，包括哪些细节被暴露、哪些细节为私有，以及程序中各个作用域中有哪些名字。Rust的这些有关代码组织的特性，有时被统称为 *模组系统（module system）*，包括了：

- **代码包（packages）**：实现代码箱（crates）的构建、测试与分享的 Cargo 特性；
- **代码箱（crates）**：产生出库或可执行文件的模组树（a tree of modules that produces a library or executable）；
- **模组（modules）**与 **`use`关键字**：实现对组织、作用域及路径私有化的控制（let you control the organization, scope, and privacy of paths）；
- **路径（paths）**：对结构体、函数或模组等进行命名的方式（a way of naming an item, such as a struct, function, or module）。


