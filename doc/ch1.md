# 添加彩色日志功能
一个合格的日志的功能，应该有一个方便的日志输出等级控制接口。然后每次输入前，判断当前的输出等级，符合条件在输出，所以很适合把日志作为一个Struct，输出作为Struct的方法，在类变量中添加日志等级

## 遇到的问题

由于我孱弱的Rust代码能力，在Tips中提到可以使用crate log，但是我没有立刻意识到crate log实际上是一个外部包，而不是标准库的包。参考rCore的代码引入log后，编译器报错`no external crate log`，因为rCore使用的是nightly的版本，开始我还以为是这个版本没有这个库，在稳定版的Rust中导入之后，仍然不能导入，终于发现是因为需要安装外部库。
## 在rCore中学到的
rCore中的log已经通过加锁，实现多线程的log，但是目前还不涉及到多线程的问题，所以我的实现中，暂时不涉及锁的部分。

Log使用时，在函数结尾放上log的级别，这个级别用于配置输出的颜色，所以需要把Log级别转换成数字，所以采用level_to_color_code函数。

虽然模仿者rCore实现了Log功能，但是在如何使用这一点上，rCore并没有给出示例，经过多次尝试与实验，最终发现
1. option_env的环境变量的配置，可以是 `LOG=trace cargo build --release` Build的时候通过见面的环境变量设置。这是通过match的时候添加print语句发现的。
2. 在使用不同的宏的时候，需要引入`use log::{error,LevelFilter,trace,info,warn};`
3. 在init后，可以通过set_max_level函数更改输出等级，也就是说，初始化的时候，根据option_env，match了一个输出等级，但这个实际上也不是必须的。
4. 经过测试，Log引入的这些宏，最终都是走的SimpleLogger实现的log方法，Record类型实际上是使用相关宏的时候，自动生成的。