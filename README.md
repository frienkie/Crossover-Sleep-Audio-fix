# Crossover Sleep-Audio-fix

[中文](#中文) · [日本語](#日本語) · [English](#english)

## 中文

### 这个软件解决什么问题？

**修复 Mac 从睡眠唤醒后，CrossOver 程序的声音回到 Mac 内置扬声器的问题。**

例如：睡眠前用显示器音箱、USB 音箱或耳机玩游戏，唤醒后声音却从 Mac 自带的扬声器传出。即使在 macOS 中重新选好了外部音响，CrossOver 里的游戏仍没有跟着切换。

本工具会让正在运行的 CrossOver 程序跟随 **macOS 当前选择的音频输出**，并在睡眠唤醒后重新检查和恢复必要的声音转发。它面向外接音响、显示器扬声器和耳机等输出切换场景。实现部分来源于CrossPuck的启发。

### 怎么使用？

1. 使用 **macOS 14.2 或更新版本**。
2. 解压应用安装包，把 **Crossover Sleep-Audio-fix.app** 放进“应用程序”并打开。若只有源代码，请参阅[构建说明](docs/DEVELOPMENT.md)。
3. 点击菜单栏的扬声器图标，开启“**跟随 macOS 音频输出**”。
4. 首次提示时，允许“**系统音频录制**”。这是转发程序声音所需的权限；声音只在内存里处理，不保存、不上传。
5. 在 macOS 的“系统设置 → 声音 → 输出”中选择你要用的音响，然后正常运行 CrossOver 程序即可。

在“**语言**”菜单中可切换中文、日文、英文；选择会被记住。升级旧版时请先退出 CrossPuck，再使用新名称的应用，避免同时运行两个版本。

### 菜单状态是什么意思？

| 状态 | 简单理解 |
| --- | --- |
| **监控中 · 音响名称** | 正在关注输出变化。程序已经从正确音响播放时，这是正常状态，不需要额外转发。 |
| **转发中 · 音响名称** | 检测到程序还在用其他设备，正在把声音转到这里。 |
| **正在连接／正在重试** | 正在恢复连接；长时间不恢复时，请检查音频录制权限和音响连接。 |

### 常见问题 / Issue

- **已修复的场景：**系统已选外部音响，CrossOver 却仍从 Mac 扬声器发声；睡眠唤醒后会重新检查并建立需要的转发。
- **已修复的识别问题：**通过 CrossOver 生成的桌面快捷方式启动的程序也能被识别。
- **需要先手动处理：**如果 macOS 默认输出本身仍是 Mac 扬声器，请先选回外部音响。本工具跟随系统的选择，不会替你指定某一台音响。
- **设备未连接或仍有问题：**先检查音响是否在 macOS 中可用。不同设备和程序仍可能有兼容性差异。

可在 [Issues](https://github.com/frienkie/Crossover-Sleep-Audio-fix/issues/new/choose) 用中文、日文或英文反馈。说明 Mac/macOS 和 CrossOver 版本、音响类型、睡眠唤醒前后从哪里发声，以及菜单显示什么即可，不需要懂代码。

## 日本語

### 何を直すアプリですか？

**Mac のスリープ復帰後、CrossOver アプリの音が Mac の内蔵スピーカーに戻ってしまう問題を改善するツールです。**

たとえば、スリープ前はモニターのスピーカー、USB スピーカー、ヘッドホンでゲームの音を聞いていたのに、復帰後は Mac 本体から音が出る場合です。macOS で外部出力を選び直しても、CrossOver のゲームだけ切り替わらないことがあります。

このツールは、実行中の CrossOver アプリの音を **macOS で現在選択している出力先**に追従させ、スリープ復帰時に接続を確認し、必要な音声転送を作り直します。実装の一部は CrossPuck から着想を得ています。

### 使い方

1. **macOS 14.2 以降**が必要です。
2. アプリの配布ファイルを解凍し、**Crossover Sleep-Audio-fix.app** を「アプリケーション」に移動して開きます。ソースコードからの作成方法は[こちら](docs/DEVELOPMENT.md)。
3. メニューバーのスピーカーアイコンから「**macOS の音声出力に追従**」をオンにします。
4. 初回の確認で「**システムオーディオの収録**」を許可します。音声転送に必要な権限です。音声はメモリ内でのみ処理され、保存やアップロードはしません。
5. macOS の「システム設定 → サウンド → 出力」で使用する機器を選び、CrossOver アプリを通常どおり使います。

「**言語**」から日本語・中国語・英語を選べます。設定は次回も保持されます。旧版から移行する場合は、先に CrossPuck を終了し、2 つのバージョンを同時に起動しないでください。

### 表示の意味

| 表示 | 意味 |
| --- | --- |
| **監視中 · 出力先** | 出力の変化を確認しています。アプリがすでに正しい機器から再生している場合も、この正常な状態になります。 |
| **転送中 · 出力先** | 別の機器に向いているアプリの音を、この出力先へ転送しています。 |
| **接続中／再試行中** | 接続を復旧しています。長く続く場合は音声収録の権限と機器の接続を確認してください。 |

### よくある問題 / Issue

- **対応した症状：**macOS は外部出力なのに CrossOver だけ内蔵スピーカーから鳴る。復帰後に必要な音声転送を作り直します。
- **対応した起動方法：**CrossOver が作成したデスクトップのショートカットから起動したアプリも識別します。
- **先に確認すること：**macOS 自体の出力先が内蔵スピーカーの場合は、外部機器を選び直してください。このツールは特定のスピーカーを選ぶのではなく、システムの選択に追従します。
- **機器が見つからない／改善しない：**まず macOS から使用できるか確認してください。機器やアプリによって相性の問題が残る場合があります。

問題は [Issues](https://github.com/frienkie/Crossover-Sleep-Audio-fix/issues/new/choose) に日本語・中国語・英語で報告できます。Mac/macOS と CrossOver のバージョン、音響機器、復帰前後の音の出る場所、メニューの表示を教えてください。プログラミングの知識は不要です。

## English

### What problem does this fix?

**This utility addresses CrossOver audio switching back to the Mac's built-in speakers after waking from sleep.**

For example, your game plays through monitor speakers, USB speakers or headphones before sleep, but through the Mac itself after wake. Selecting the external output again in macOS may still leave the CrossOver game playing through the built-in speakers.

The app keeps running CrossOver programs following **the output currently selected in macOS**. After wake, it checks the connection and rebuilds audio forwarding when needed. Parts of the implementation are inspired by CrossPuck.

### How to use it

1. Use **macOS 14.2 or later**.
2. Unzip the app package, move **Crossover Sleep-Audio-fix.app** to Applications and open it. If you only have source code, see the [build guide](docs/DEVELOPMENT.md).
3. Click the speaker icon in the menu bar and enable **Follow macOS Audio Output**.
4. Allow **System Audio Recording** when prompted. This permission is needed to forward application audio. Audio stays in memory and is never saved or uploaded.
5. Select your speakers in macOS **System Settings → Sound → Output**, then use your CrossOver programs as usual.

Use **Language** to switch between English, Chinese and Japanese. Your choice is remembered. When upgrading, quit the older CrossPuck app before opening the renamed app so that two versions do not run together.

### What do the status messages mean?

| Status | Plain-language meaning |
| --- | --- |
| **Monitoring · Output** | Watching for output changes. This is also normal when your program already plays through the right device and needs no forwarding. |
| **Rerouting · Output** | Forwarding a program's audio from a different device to this output. |
| **Connecting / Retrying** | Recovering the connection. If this persists, check audio permission and the device connection. |

### Common problems / Issues

- **Addressed symptom:** macOS selects an external output, but CrossOver still plays through the Mac speakers. The utility rechecks and rebuilds forwarding after wake when needed.
- **Addressed discovery problem:** apps launched through CrossOver-generated desktop shortcuts are recognized too.
- **Check first:** if macOS itself still selects the built-in speakers, choose your external device. This app follows the system selection; it does not pick a particular speaker for you.
- **Missing device or continuing trouble:** check that macOS can use the device. Compatibility can still vary by device and application.

Report problems in English, Chinese or Japanese through [Issues](https://github.com/frienkie/Crossover-Sleep-Audio-fix/issues/new/choose). Include your Mac/macOS and CrossOver versions, speaker connection type, where audio plays before and after wake, and the menu status. No coding knowledge is needed.

---

Based on the audio-routing work in [scryner/crosspuck](https://github.com/scryner/crosspuck), licensed under Apache-2.0. This fork focuses on audio only; controller support and automatic updates are removed. [Developer guide](docs/DEVELOPMENT.md) · [Audio design](docs/audio-routing.md)
