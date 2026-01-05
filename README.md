# 迷宫寻路算法 (Rust)

<div align="center">

**使用 Rust + egui 构建的迷宫生成与寻路可视化工具**

![Rust](https://img.shields.io/badge/Rust-2021-orange?logo=rust)
![egui](https://img.shields.io/badge/GUI-egui-blue)
![License](https://img.shields.io/badge/license-MIT-green)

</div>

---

## ✨ 功能特性

- 🏗️ **Kruskal + 并查集** 生成完美迷宫（保证起点到终点有唯一解）
- 🔍 四种经典寻路算法：DFS / BFS / DBFS / A*
- 🎬 逐步可视化搜索过程，支持前进/后退
- ⚡ 可调节动画速度，批量步进提升效率
- 🖥️ 原生 GUI，无需浏览器或外部运行时

---

## 🚀 快速开始

### 环境要求

- Rust 1.70+（推荐通过 [rustup](https://rustup.rs/) 安装）

### 构建 & 运行

```bash
# 克隆或进入项目目录
cd maze-routing-algorithm-rust

# 编译并运行（Release 模式）
cargo run --release --bin maze-gui
```

---

## 🎮 操作说明

### 快捷键

| 按键 | 功能 |
|------|------|
| `Space` | 开始 / 暂停搜索 |
| `R` | 重置地图（保留迷宫） |
| `N` | 生成新迷宫 |
| `←` / `→` | 单步后退 / 前进 |
| `1` `2` `3` `4` | 切换算法：DFS / BFS / DBFS / A* |
| `+` / `-` | 加速 / 减速动画 |
| `Esc` | 退出 |

### 颜色图例

| 颜色 | 含义 |
|------|------|
| ⬛ 黑色 | 墙壁 |
| ⬜ 白色 | 通道 |
| 🟡 黄色 | 起点 / 终点 |
| 🔵 天蓝 | 已访问 |
| 🔴 浅红 | 回溯 |
| 🟢 青绿 | 最终路径 |

---

## 📂 项目结构

```
src/
├── main.rs          # GUI 入口
├── lib.rs           # 库导出
├── config/          # 常量配置（地图尺寸、颜色等）
├── core/
│   ├── map.rs       # 地图生成（Kruskal + 并查集）
│   └── point.rs     # 坐标 & 颜色定义
├── alg/
│   ├── dfs.rs       # 深度优先搜索
│   ├── bfs.rs       # 广度优先搜索
│   ├── dbfs.rs      # 双向 BFS
│   └── astar.rs     # A* 启发式搜索
├── render/          # 纹理渲染 & 路径绘制
├── ui/              # 控制面板 & 状态显示
└── input/           # 键盘快捷键处理
```

---

## 🧠 算法简介

| 算法 | 特点 | 最优路径 | 时间复杂度 |
|------|------|:--------:|------------|
| **DFS** | 深入优先，快速但路径可能较长 | ❌ | $O(V+E)$ |
| **BFS** | 层级扩展，保证最短路径 | ✅ | $O(V+E)$ |
| **DBFS** | 双向同时搜索，相遇即结束 | ✅ | $O(V+E)$ |
| **A*** | 启发式 $f=g+h$，高效且最优 | ✅ | $O(E \log V)$ |

> $V$：节点数，$E$：边数；A* 使用曼哈顿距离作为启发函数。

---

## 🛠️ 迷宫生成

采用 **Kruskal 最小生成树 + 并查集（路径压缩 + 按秩合并）** 算法：

1. 将所有单元格初始化为墙
2. 奇数坐标 $(2k+1, 2j+1)$ 作为潜在通道
3. 随机打乱相邻单元格之间的墙
4. 逐墙判断：若两侧单元格不连通，则打通该墙并合并集合
5. 最终生成无环的完美迷宫

---

## 🖼️ 界面布局

基于 `ui/mod.rs` 实现的三面板布局设计（即时模式 GUI）：

<div style="display: grid; grid-template-columns: 1fr; gap: 20px; max-width: 1000px; margin: 20px 0;">
  <!-- Header -->
  <div style="background: linear-gradient(90deg, #2a5cdb, #1e90ff); color: white; padding: 15px; border-radius: 8px; font-weight: bold; text-align: center; animation: slideDown 0.8s ease;">
    🔍 迷宫寻路算法演示 (Rust + egui v1.5.0)
  </div>

  <!-- Main Layout -->
  <div style="display: grid; grid-template-columns: 230px 1fr 230px; gap: 15px; animation: fadeIn 1s ease;">
    
    <!-- Left Panel -->
    <div style="background: #f5f5f5; border: 2px solid #ddd; border-radius: 8px; padding: 15px; overflow-y: auto; animation: slideInLeft 0.8s ease;">
      <h4 style="margin-top: 0; color: #333;">⚙️ 控制</h4>
      <hr style="margin: 10px 0; border: none; border-top: 1px solid #ccc;">
      
      <div style="margin-bottom: 15px;">
        <strong>🎯 选择算法</strong>
        <div style="margin: 8px 0; padding: 5px; cursor: pointer; transition: all 0.3s;">○ 🔸 DFS (深度优先)</div>
        <div style="margin: 8px 0; padding: 5px; cursor: pointer; background: #e3f2fd; border-radius: 4px; transition: all 0.3s;">● 🔹 BFS (广度优先)</div>
        <div style="margin: 8px 0; padding: 5px; cursor: pointer; transition: all 0.3s;">○ 🔷 DBFS (双向BFS)</div>
        <div style="margin: 8px 0; padding: 5px; cursor: pointer; transition: all 0.3s;">○ ⭐ A* (启发式)</div>
      </div>

      <hr style="margin: 10px 0; border: none; border-top: 1px solid #ccc;">

      <div style="margin-bottom: 15px;">
        <strong>🎮 操作</strong>
        <div style="margin: 5px 0;"><button style="width: 100%; padding: 6px; background: #4CAF50; color: white; border: none; border-radius: 4px; cursor: pointer; transition: all 0.2s; font-weight: bold;">▶ 开始搜索</button></div>
        <div style="margin: 5px 0;"><button style="width: 100%; padding: 6px; background: #2196F3; color: white; border: none; border-radius: 4px; cursor: pointer; transition: all 0.2s;">⏸ 暂停/继续</button></div>
        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 5px;">
          <button style="padding: 6px; background: #FF9800; color: white; border: none; border-radius: 4px; cursor: pointer;">⏮ 上一步</button>
          <button style="padding: 6px; background: #FF9800; color: white; border: none; border-radius: 4px; cursor: pointer;">⏭ 下一步</button>
        </div>
        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 5px; margin-top: 5px;">
          <button style="padding: 6px; background: #f44336; color: white; border: none; border-radius: 4px; cursor: pointer;">🔄 重置</button>
          <button style="padding: 6px; background: #9C27B0; color: white; border: none; border-radius: 4px; cursor: pointer;">🎲 新地图</button>
        </div>
      </div>

      <hr style="margin: 10px 0; border: none; border-top: 1px solid #ccc;">

      <div>
        <strong>⏱️ 播放参数</strong>
        <div style="margin: 8px 0; font-size: 12px;">
          动画速度: <span style="font-weight: bold;">10 ms/步</span>
          <div style="width: 100%; height: 6px; background: #ddd; border-radius: 3px; margin: 5px 0; overflow: hidden;">
            <div style="width: 60%; height: 100%; background: linear-gradient(90deg, #4CAF50, #81C784); animation: expand 1.5s infinite;"></div>
          </div>
        </div>
        <div style="margin: 8px 0; font-size: 12px;">
          每帧步数: <span style="font-weight: bold;">5</span>
          <div style="width: 100%; height: 6px; background: #ddd; border-radius: 3px; margin: 5px 0; overflow: hidden;">
            <div style="width: 30%; height: 100%; background: linear-gradient(90deg, #2196F3, #64B5F6); animation: pulse 2s infinite;"></div>
          </div>
        </div>
        <div style="margin: 8px 0;">
          <label style="display: flex; align-items: center; cursor: pointer;">
            <input type="checkbox" checked style="margin-right: 8px;">
            <span>🔀 显示路径箭头</span>
          </label>
        </div>
      </div>
    </div>

    <!-- Center Panel - Maze -->
    <div style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); border-radius: 8px; padding: 15px; display: flex; align-items: center; justify-content: center; min-height: 400px; position: relative; overflow: hidden; animation: slideInUp 0.8s ease;">
      <div style="font-size: 32px; opacity: 0.3; animation: float 3s ease-in-out infinite;">迷宫区域</div>
      <!-- Animated maze grid -->
      <div style="position: absolute; top: 20px; left: 20px; display: grid; grid-template-columns: repeat(8, 20px); gap: 2px; animation: fadeIn 1.2s ease;">
        <div style="width: 20px; height: 20px; background: #000;"></div>
        <div style="width: 20px; height: 20px; background: #000;"></div>
        <div style="width: 20px; height: 20px; background: #000;"></div>
        <div style="width: 20px; height: 20px; background: #000;"></div>
        <div style="width: 20px; height: 20px; background: #000;"></div>
        <div style="width: 20px; height: 20px; background: #000;"></div>
        <div style="width: 20px; height: 20px; background: #000;"></div>
        <div style="width: 20px; height: 20px; background: #000;"></div>
        <div style="width: 20px; height: 20px; background: #000;"></div>
        <div style="width: 20px; height: 20px; background: #FFD700; animation: pulse 1s infinite;"></div>
        <div style="width: 20px; height: 20px; background: #87CEEB; animation: slideInRight 0.5s ease;"></div>
        <div style="width: 20px; height: 20px; background: #87CEEB; animation: slideInRight 0.6s ease;"></div>
        <div style="width: 20px; height: 20px; background: #FFF;"></div>
        <div style="width: 20px; height: 20px; background: #000;"></div>
        <div style="width: 20px; height: 20px; background: #FFF;"></div>
        <div style="width: 20px; height: 20px; background: #000;"></div>
      </div>
    </div>

    <!-- Right Panel -->
    <div style="background: #f5f5f5; border: 2px solid #ddd; border-radius: 8px; padding: 15px; overflow-y: auto; animation: slideInRight 0.8s ease;">
      <h4 style="margin-top: 0; color: #333;">📊 信息</h4>
      <hr style="margin: 10px 0; border: none; border-top: 1px solid #ccc;">

      <div style="margin-bottom: 15px; font-size: 13px;">
        <div><strong>当前算法:</strong> BFS</div>
        <div><strong>状态:</strong> <span style="color: #4CAF50; font-weight: bold; animation: blink 1s infinite;">运行中</span></div>
        <div><strong>自动播放:</strong> 是</div>
        <div><strong>步骤:</strong> 234 / 1000</div>
        <div><strong>找到路径:</strong> ✅ 是</div>
        <div><strong>路径长度:</strong> 89</div>
        <div><strong>耗时:</strong> 45 ms</div>
      </div>

      <hr style="margin: 10px 0; border: none; border-top: 1px solid #ccc;">

      <div style="margin-bottom: 15px;">
        <strong>🎨 颜色说明</strong>
        <div style="margin: 6px 0; display: flex; align-items: center;">
          <div style="width: 16px; height: 16px; background: #000; border-radius: 2px; margin-right: 8px;"></div>
          <span style="font-size: 12px;">墙壁</span>
        </div>
        <div style="margin: 6px 0; display: flex; align-items: center;">
          <div style="width: 16px; height: 16px; background: #FFF; border: 1px solid #999; border-radius: 2px; margin-right: 8px;"></div>
          <span style="font-size: 12px;">通道</span>
        </div>
        <div style="margin: 6px 0; display: flex; align-items: center;">
          <div style="width: 16px; height: 16px; background: #FFD700; border-radius: 2px; margin-right: 8px;"></div>
          <span style="font-size: 12px;">起点/终点</span>
        </div>
        <div style="margin: 6px 0; display: flex; align-items: center;">
          <div style="width: 16px; height: 16px; background: #87CEEB; border-radius: 2px; margin-right: 8px;"></div>
          <span style="font-size: 12px;">已访问</span>
        </div>
        <div style="margin: 6px 0; display: flex; align-items: center;">
          <div style="width: 16px; height: 16px; background: #FF6A6A; border-radius: 2px; margin-right: 8px;"></div>
          <span style="font-size: 12px;">回溯</span>
        </div>
        <div style="margin: 6px 0; display: flex; align-items: center;">
          <div style="width: 16px; height: 16px; background: #7FFFD4; border-radius: 2px; margin-right: 8px;"></div>
          <span style="font-size: 12px;">最终路径</span>
        </div>
      </div>

      <hr style="margin: 10px 0; border: none; border-top: 1px solid #ccc;">

      <div>
        <strong>⌨️ 快捷键</strong>
        <div style="font-size: 11px; margin-top: 8px;">
          <div><code>Space</code> 开始/暂停</div>
          <div><code>R</code> 重置地图</div>
          <div><code>N</code> 生成新地图</div>
          <div><code>←/→</code> 上一步/下一步</div>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  @keyframes slideDown { from { transform: translateY(-20px); opacity: 0; } to { transform: translateY(0); opacity: 1; } }
  @keyframes slideInLeft { from { transform: translateX(-20px); opacity: 0; } to { transform: translateX(0); opacity: 1; } }
  @keyframes slideInRight { from { transform: translateX(20px); opacity: 0; } to { transform: translateX(0); opacity: 1; } }
  @keyframes slideInUp { from { transform: translateY(20px); opacity: 0; } to { transform: translateY(0); opacity: 1; } }
  @keyframes fadeIn { from { opacity: 0; } to { opacity: 1; } }
  @keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.7; } }
  @keyframes blink { 0%, 100% { opacity: 1; } 50% { opacity: 0.5; } }
  @keyframes float { 0%, 100% { transform: translateY(0px); } 50% { transform: translateY(-10px); } }
  @keyframes expand { 0% { transform: scaleX(0); } 100% { transform: scaleX(1); } }
  button:hover { transform: translateY(-2px); box-shadow: 0 4px 8px rgba(0,0,0,0.2) !important; }
</style>

**布局说明（源码映射）：**

| 区域 | 源码位置 | 尺寸 | 功能 |
|------|---------|------|------|
| **顶部标题栏** | `ui/mod.rs::top_bar()` | 全宽 | 显示程序标题、版本号 |
| **左侧控制面板** | `ui/mod.rs::left_panel()` | 230px | 算法选择、操作按钮、播放参数 |
| **中央迷宫区域** | `render/mod.rs::sync_texture()` | 自适应 | 迷宫纹理渲染、路径箭头叠加 |
| **右侧信息面板** | `ui/mod.rs::right_panel()` | 230px | 统计信息、颜色说明、快捷键提示 |

---

## ⚖️ 与 C++ Qt 版本对比

| 对比项 | C++ / Qt6 | Rust / egui |
|--------|-----------|-------------|
| **语言** | C++17 | Rust 2021 |
| **GUI 框架** | Qt6 Widgets | egui (即时模式 GUI) |
| **渲染方式** | QPainter 逐帧绘制 | GPU 纹理 + 即时模式 |
| **线程模型** | QThread 多线程搜索 | 单线程预计算 + 批量步进 |
| **内存管理** | 手动 new/delete | 所有权系统自动管理 |
| **构建工具** | XMake / CMake | Cargo |
| **依赖管理** | 手动或 vcpkg | crates.io 自动拉取 |
| **跨平台支持** | 需分别配置工具链 | `cargo build` 原生支持 |
| **可执行体积** | ~15 MB (含 Qt DLL) | ~3 MB (静态链接) |
| **迷宫生成算法** | Prim 随机生成 | Kruskal + 并查集 |
| **热键处理** | Qt 信号槽机制 | egui Input 事件轮询 |
| **中文字体支持** | QString 原生 | 内嵌 TTF 字体文件 |

### 性能与优势对比

| Rust 版本优势 | C++ 版本优势 |
|---------------|--------------|
| ✅ 无运行时依赖，开箱即用 | ✅ 成熟的 Qt 生态、丰富文档 |
| ✅ 内存安全，无数据竞争风险 | ✅ 多线程搜索更灵活可控 |
| ✅ 编译即优化，性能接近 C++ | ✅ IDE 支持完善（Qt Creator） |
| ✅ 增量编译快，开发效率高 | ✅ 更丰富的预设 UI 组件库 |
| ✅ 跨平台最小化配置 | ✅ 企业级应用成熟度高 |

---

## 📜 许可证

MIT License

