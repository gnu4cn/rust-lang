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


