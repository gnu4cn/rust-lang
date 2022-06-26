# 运用结构体来建构相关数据

**Using Structs to Structure Related Data**

*结构体（struct）*，或者说 *结构（structure）*，实现了将多个相关值打包在一起，并取个名字，而构成一个有意义的组别。在熟悉面向对象语言的情况下，那么 *结构体* 就像是对象的那些数据属性。在本章中，将把元组与结构体加以比照，从而在既有认识之上，构建出对结构体的认识，并对使用结构体作为一种更佳的数据组织方式的时机，进行演示。这里会对如何定义及初始化结构体进行演示。还会讨论如何定义关联函数，尤其是那种叫做 *方法* 的关联函数，来指明与某个结构体类型相关联的行为。结构体与枚举（将在第 6 章讨论到），这两种数据结构，是充分利用 Rust 的编译时类型检查特性，在程序域中创建新类型的构件。

## 结构体的定义及初始化

结构体与之前 [元组类型](Ch03_Common_Programming_Concepts.md#the-tuple-type) 小节中讨论过的元组数据结构类似，二者都保存着多个相关数据。和元组一样，结构体的各个数据片段可以是不同类型。与原则不同的是，在结构体中将给各个数据片段命名，如此各个值表示什么就清楚了。加上这些名字，就意味着相比于元组更为灵活了：不必为了给某个实例指定他的那些值，或要访问实例的那些值，而对实例数据的顺序有所依赖了。

要定义出一个结构体，就要敲入关键字 `struct`，及整个结构体的名字。结构体名字，应对安排在一起的这些数据片段的意义加以描述。随后，就要这一对花括号里头，定义出各个数据片段的名称与类型，这些数据片段，就叫做 *字段（fields）*。比如，下面的清单 5-1 就给出了一个保存用户账号信息的结构体。

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}
```

*清单 5-1：`User` 结构体的定义*

在定义出了结构体后，要用上这个结构体，就要通过给各个字段指定具体值，创建出那个结构体的 *实例（instance）* 来。通过指明结构的名字，并随后加上包含了 `key: value` 键值对的一对花括号，这样创建出一个实例来。键值对中的那些键，就是那些字段的名字，而其中的那些值，则是打算保存在这些字段中的数据。不必按照在结构体中声明那些字段的顺序，来对这些字段进行指明（we don't have to specify the fields in the same order in which we declared them in the struct）。也就是说，结构体定义就如同该类型的通用模板，而实例则将特定数据填充到那个木板中，从而创建出这个类型的值来。比如，就可如下面清单 5-2 中所展示的那样，声明出一个特定的用户来：

```rust
fn main() {
    let user1 = User {
        email: String::from("rust@xfoss.com"),
        username: String::from("unisko"),
        active: true,
        sign_in_count: 1
    };
}
```

*清单 5-2：创建出结构体 `User` 的一个实例来*

而要从结构体中获取到指定值，就要使用点表示法（`.`）。在要的仅是该用户的电子邮件地址时，就可以在那些要用到这个值的地方，使用 `user1.email` 。而在该实例为可变时，那么就可以通过使用点表示法，进而给特定字段赋值，而对某个值加以修改。下面的清单 5-3 展示了如何来修改某个可变 `User` 实例 `email` 字段中的值。

文件名：`src/main.rs`

```rust
fn main() {
    let mut user1 = User {
        email: String::from("rust@xfoss.com"),
        username: String::from("unisko"),
        active: true,
        sign_in_count: 1
    };

    user1.email = String::from("java@xfoss.com");
}
```

*清单 5-3：对某个 `User` 实例中的 `email` 字段进行修改*

请注意这整个实例必须是可变的；Rust 不允许仅将一些字段标记为可变。与所有表达式一样，可以函数体中最后的表达式形式，构造出结构体的新实例，来隐式地返回那个新实例（as with any expression, we can construct a new instance of the struct as the last expression in the function body to implicity return that new instance）。

下面的清单 5-4，展示了一个以给定电子邮件和用户名，返回一个 `User` 实例的 `build_user` 函数。其中的 `active` 字符会得到值 `true`，而那个 `sign_in_count` 则会得到值 `1`。

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

*清单 5-4：一个取得电子邮件和用户名，并返回一个 `User` 实例的 `build_user` 函数*

将函数参数命名为与结构体字段同样的名字，是有意义，但由此而不得不重复那 `email` 与 `username` 的字段名字与变量，就有点烦人了。在结构体有更多字段时，这样重复各个名字就会变得更加烦人。幸运的是，有种方便的简写法！


### 使用字段初始化简写法

由于在清单 5-4 中的参数名字与结构体字段名字完全一样，因此就可以 *字段初始化简写（field init shorthand）* 语法，来重写 `build_user` 方法，如此一来，`build_user` 函数在没有 `email` 与 `username` 重复的情况下，也有与之前版本同样的表现，如下清单 5-5 所示：

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

*清单 5-5：由于 `email` 与 `username` 参数与结构体字段有着同样名字，而使用了字段初始化简写的 `build_user` 函数*

在这里，正创建一个 `User` 结构体的新实例，该结构体有一个名为 `email` 的字段。这里打算将 `email` 字段的值，设置为 `build_user` 函数的 `email` 参数中的值。由于 `email` 字段与 `email` 参数有着同样的名字，因此只就需写下 `email`，而非 `email: email`。


### 使用结构体更新语法，从其他实例创建出实例

创建出包含另一实例绝大部分值，而修改一些值的新实例，通常是有用的做法。而使用 *结构体更新语法（struct update syntax）* 就能做到这点。

首先，在下面的清单 5-6 中展示了如何按常规，不使用更新语法的情况下，创建出在 `user2` 中的一个新 `User` 实例。这里给 `email` 设置了一个新的值，而在其他方面，则使用了来自之前在清单 5-1 中创建的 `user1` 的那些同样值。

```rust
fn main() {
    // --跳过代码--

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("java@xfoss.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```

*清单 5-6：使用一个 `user1` 的值创建出一个新的 `User` 实例*

而使用结构体更新语法，就可以较少代码，达成同样效果，如下面的清单 5-7 中所给出的那样。其中的 `..` 语法，指明了未显式设置的其余字段，将有着与所给实例中的字段同样的值。

```rust
fn main() {
    // --跳过代码--

    let user2 = User {
        email: String::from("java@xfoss.com"),
        ..user1
    };
}
```

*清单 5-7：使用结构体更新语法来设置 `User` 实例的 `email` 字段值，而使用来自 `user1` 的其余值*

清单 5-7 中的代码同样创建了在变量 `user2` 中，一个有着 `email` 的不同值，但有着来自 `user1` 的 `username`、`active` 及 `sign_in_count` 同样值。
