# AutoControl
`autocontrol` 是一个基于 Rust 开发的 Windows 自动化控制库，
它封装了 Windows 原生 API，提供简单、安全、高性能的自动化接口。

## 主要功能
- 鼠标控制
- 键盘控制
- 组合键
- 文本输入
- 剪贴板操作
- 屏幕信息获取
- 全屏截图
- 区域截图

## 安装
运行 `cargo add autocontrol` 
或者在Cargo.toml文件中添加`autocontrol`

## 使用方法
导入并初始化AutoControl

```rust
use autocontrol::AutoControl;

fn main() {
    let auto = AutoControl::new();
}
```
## 鼠标控制
- 获取当前鼠标屏幕坐标
```rust 
    let (x, y) = auto.position().unwrap();
    println!("x={x},y={y}");
```
- 左键单击
```rust
    auto.left_click().unwrap();
```
- 右键单击
```rust
    auto.right_click().unwrap();
```
- 中键单击
```rust
    auto.middle_click().unwrap();
```
- 按下和释放鼠标左键
```rust
    auto.left_down().unwrap();
    auto.left_up().unwrap();
```
- 鼠标双击
```rust
    auto.double_click().unwrap();
```
- 相对移动鼠标
```rust
    // 鼠标向右移动100像素,向下移动50像素
    auto.move_relative(100, 50).unwrap();
```
- 移动鼠标到指定屏幕坐标
```rust
    // 鼠标移动到(200，100)坐标点
    auto.move_absolute(200, 100).unwrap();
```
- 垂直滚动鼠标滚轮
```rust
    //滚轮向下移动，一个标准滚轮单位通常为 `120`
    auto.scroll_vertical(-120).unwrap();
```
- 水平滚动鼠标滚轮
```rust
    //滚轮向左移动，一个标准滚轮单位通常为 `120`
    auto.scroll_horizontal(-120).unwrap();
```
- 平滑移动鼠标到目标位置
```rust
    // 1秒移动到(800,600)位置
    auto.smooth_move(800, 600, 1000).unwrap();
```
- 拖拽鼠标
```rust
    // 鼠标1秒钟从(50, 50)拖拽到(200, 200)位置
    auto.drag((50, 50), (200, 200), 1000).unwrap();
```
- 暂停等待
```rust
    //等待1秒
    auto.wait(1000).unwrap();
```
## 键盘控制
- 按下键盘按键
```rust
    // 使用枚举方式
    use autocontrol::Key;
    auto.key_down(Key::A).unwrap();
    
    //使用字符串方式
    auto.key_down_str("b").unwrap();
```
- 释放键盘按键
```rust
    // 使用枚举方式
    use autocontrol::Key;
    auto.key_up(Key::A).unwrap();
    
    //使用字符串方式
    auto.key_up_str("b").unwrap();
```
- 按下并释放键盘按键
```rust
    // 使用枚举方式
    use autocontrol::Key;
    auto.press(Key::A).unwrap();

    //使用字符串方式
    auto.press_str("b").unwrap();
```
- 执行组合键
```rust
    // 使用枚举方式
    use autocontrol::Key;
    auto.combo([Key::Ctrl, Key::C]).unwrap();

    //使用字符串方式
    auto.combo_str(["ctrl", "v"]).unwrap();
```

- 输入文本
```rust
    auto.text("hello rust!").unwrap();
    auto.text("你好，🦀!").unwrap();
```
## 屏幕操作
- 获取主显示器宽度
```rust
    let width = auto.screen_width().unwrap();
    println!("{width}");
```
- 获取主显示器高度
```rust
    let height = auto.screen_height().unwrap();
    println!("{height}");
```
- 获取主显示器尺寸
```rust
    let (width, height) = auto.screen_size().unwrap();
    println!("width={width},height={height}");
```
- 全屏截图
```rust
    //全屏截图并保存为full.png
    auto.screenshot().unwrap().save("full.png").unwrap();
```
- 区域截图
```rust
    // 从(100,100)位置开始截取宽500高300的区域截图并保存为region.png
    auto.screenshot_region(100, 100, 500, 300).unwrap().save("region.png").unwrap();
```

## 剪贴板操作
- 获取剪贴板文本 
· 剪贴板为空时报错未处理，慎用
```rust
    let text = auto.clipboard_get_text().unwrap();
    println!("{text}");
```
- 设置剪贴板文本
```rust
    auto.clipboard_set_text("你好，🦀！").unwrap();
```
- 清空剪贴板内容
```rust
    auto.clipboard_clear().unwrap();
```
- 获取剪贴板图片
· 剪贴板为空时报错未处理，慎用
```rust
    let image = auto.clipboard_get_image().unwrap();
```
- 设置剪贴板图片
```rust
    auto.clipboard_set_image(&image).unwrap();
```
