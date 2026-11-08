# Crossover Sleep-Audio-fix

[中文](#中文) · [日本語](#日本語) · [English](#english)

[下载 DMG / DMG をダウンロード / Download DMG](https://github.com/frienkie/Crossover-Sleep-Audio-fix/releases/latest)

## 中文

### 这个软件解决什么问题？

**修复 Mac 从睡眠唤醒后，CrossOver 程序的声音回到 Mac 内置扬声器的问题。**

例如：睡眠前用显示器音箱、USB 音箱或耳机玩游戏，唤醒后声音却从 Mac 自带的扬声器传出。即使在 macOS 中重新选好了外部音响，CrossOver 里的游戏仍没有跟着切换。

本工具会让正在运行的 CrossOver 程序跟随 **macOS 当前选择的音频输出**，并在睡眠唤醒后重新检查和恢复必要的声音转发。它面向外接音响、显示器扬声器和耳机等输出切换场景。实现部分来源于CrossPuck的启发。

### 怎么使用？

1. 使用 **macOS 14.2 或更新版本**。本次发布的 DMG 适用于 **Apple Silicon（M 系列）Mac**。
2. 打开下载的 DMG，把 **Crossover Sleep-Audio-fix.app** 拖入“应用程序”并打开。若只有源代码，请参阅[构建说明](docs/DEVELOPMENT.md)。
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

### 在原corsspuck基础上修复的问题 

- **已修复的场景**：系统已选外部音响，CrossOver 却仍从 Mac 扬声器发声；睡眠唤醒后会重新检查并建立需要的转发。
- **已修复的识别问题**：通过 CrossOver 生成的桌面快捷方式启动的程序也能被识别。

### 故障排查

- **请确定系统默认音响**：如果 macOS 默认输出本身仍是 Mac 扬声器，请先选回外部音响。本工具跟随系统的选择，不会替你指定某一台音响。
- **设备未连接或仍有问题**：先检查音响是否在 macOS 中可用。不同设备和程序仍可能有兼容性差异。

可在 [Issues](https://github.com/frienkie/Crossover-Sleep-Audio-fix/issues/new/choose) 用中文、日文或英文反馈。说明 Mac/macOS 和 CrossOver 版本、音响类型、睡眠唤醒前后从哪里发声，以及菜单显示什么即可，不需要懂代码。

## 日本語

### このソフトはどのような問題を直しますか？

**Mac のスリープ復帰後、CrossOver アプリの音声出力が Mac の内蔵スピーカーに戻ってしまう問題を修正します。**

たとえば、スリープ前はモニターのスピーカー、USB スピーカー、ヘッドホンでゲームの音を聞いていたのに、復帰後は Mac 本体のスピーカーから音が出る場合です。macOS で外部スピーカーなどを選び直しても、CrossOver のゲームは出力先が切り替わらないことがあります。

このツールは、実行中の CrossOver アプリの音声出力を **macOS で現在選択している出力先**に追従させ、スリープ復帰後に再確認し、必要な音声転送を復旧します。外部スピーカー、モニターのスピーカー、ヘッドホンなどの出力切り替えに対応するためのツールです。実装の一部は CrossPuck から着想を得ています。

### 使い方

1. **macOS 14.2 以降**を使用してください。今回の DMG は **Apple Silicon（M シリーズ）Mac** 用です。
2. ダウンロードした DMG を開き、**Crossover Sleep-Audio-fix.app** を「アプリケーション」にドラッグして開きます。ソースコードしかない場合は、[ビルド手順](docs/DEVELOPMENT.md)を参照してください。
3. メニューバーのスピーカーアイコンをクリックし、「**macOS の音声出力に追従**」をオンにします。
4. 初回の確認で「**システムオーディオの収録**」を許可します。これはアプリの音声を転送するために必要な権限です。音声はメモリ内でのみ処理され、保存やアップロードはしません。
5. macOS の「システム設定 → サウンド → 出力」で使用するスピーカーなどを選び、CrossOver アプリを通常どおり使います。

「**言語**」メニューで中国語・日本語・英語を切り替えられます。選択は次回も保持されます。旧版から移行する場合は、先に CrossPuck を終了してから新しい名前のアプリを使い、2 つのバージョンが同時に起動しないようにしてください。

### メニューの表示は何を意味しますか？

| 表示 | わかりやすい説明 |
| --- | --- |
| **監視中 · 出力先** | 出力の変化を確認しています。アプリがすでに正しいスピーカーなどから再生している場合は正常な状態で、追加の音声転送は不要です。 |
| **転送中 · 出力先** | アプリがまだ別の機器を使用していることを検出し、音声をこの出力先へ転送しています。 |
| **接続中／再試行中** | 接続を復旧しています。長く続く場合は、音声収録の権限と機器の接続を確認してください。 |

### 元の CrossPuck をベースに修正した問題

- **修正済みの症状**：システムで外部スピーカーなどを選択しているのに、CrossOver の音は Mac のスピーカーから出る問題です。スリープ復帰後に再確認し、必要な音声転送を作成します。
- **修正済みの識別問題**：CrossOver が作成したデスクトップのショートカットから起動したアプリも識別できるようになりました。

### トラブルシューティング

- **システムのデフォルト出力先を確認してください**：macOS 自体の出力先がまだ Mac のスピーカーになっている場合は、先に外部機器を選び直してください。このツールはシステムの選択に追従するもので、特定のスピーカーを代わりに選ぶものではありません。
- **機器が接続されていない、または問題が続く場合**：まず、その機器が macOS から使用できるか確認してください。機器やアプリによって互換性に違いがある場合があります。

[Issues](https://github.com/frienkie/Crossover-Sleep-Audio-fix/issues/new/choose) に中国語・日本語・英語で報告できます。Mac/macOS と CrossOver のバージョン、音響機器の種類、スリープ復帰前後にどこから音が出るか、メニューに何が表示されるかを教えてください。プログラミングの知識は不要です。

## English

### What problem does this software fix?

**It fixes CrossOver audio switching back to the Mac's built-in speakers after the Mac wakes from sleep.**

For example, you play a game through monitor speakers, USB speakers or headphones before sleep, but after wake the sound comes from the Mac's built-in speakers. Even after you select the external output again in macOS, the CrossOver game may still fail to switch with it.

This tool keeps the audio output of running CrossOver programs following **the output currently selected in macOS**. After wake, it rechecks and restores audio forwarding when needed. It is intended for switching between external speakers, monitor speakers, headphones and similar outputs. Parts of the implementation are inspired by CrossPuck.

### How to use it

1. Use **macOS 14.2 or later**. This DMG release is for **Apple Silicon (M-series) Macs**.
2. Open the downloaded DMG, drag **Crossover Sleep-Audio-fix.app** into Applications and open it. If you only have source code, see the [build guide](docs/DEVELOPMENT.md).
3. Click the speaker icon in the menu bar and enable **Follow macOS Audio Output**.
4. Allow **System Audio Recording** when first prompted. This permission is needed to forward application audio. Audio is processed only in memory and is never saved or uploaded.
5. Select the speakers or other output you want in macOS **System Settings → Sound → Output**, then use your CrossOver programs as usual.

Use the **Language** menu to switch between Chinese, Japanese and English. Your choice is remembered. When upgrading, quit CrossPuck before using the app under its new name to avoid running two versions at the same time.

### What do the menu status messages mean?

| Status | Plain-language meaning |
| --- | --- |
| **Monitoring · Output** | Watching for output changes. When your program already plays through the correct device, this is normal and no additional forwarding is needed. |
| **Rerouting · Output** | The tool has detected that a program is still using another device and is forwarding its audio to this output. |
| **Connecting / Retrying** | Recovering the connection. If this persists, check audio recording permission and the device connection. |

### Problems fixed on top of the original CrossPuck

- **Fixed playback scenario**: the system selects an external output, but CrossOver still plays through the Mac speakers. After wake, the tool rechecks and sets up forwarding when needed.
- **Fixed application detection**: programs launched through CrossOver-generated desktop shortcuts can now be recognized too.

### Troubleshooting

- **Check the system's default audio output**: if macOS itself still selects the Mac speakers, first switch back to your external device. This tool follows the system's selection; it does not choose a particular speaker for you.
- **Device disconnected or problems persist**: first check that the audio device is available in macOS. Compatibility can still vary between devices and programs.

Report problems in Chinese, Japanese or English through [Issues](https://github.com/frienkie/Crossover-Sleep-Audio-fix/issues/new/choose). Include your Mac/macOS and CrossOver versions, audio device type, where the sound comes from before and after wake, and what the menu displays. No coding knowledge is needed.

---

Based on the audio-routing work in [scryner/crosspuck](https://github.com/scryner/crosspuck), licensed under Apache-2.0. This fork focuses on audio only; controller support and automatic updates are removed. [Developer guide](docs/DEVELOPMENT.md) · [Audio design](docs/audio-routing.md)
