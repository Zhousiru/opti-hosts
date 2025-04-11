# Opti Hosts：拼好 Hosts

[[English]](./README.md) [[中文]](./README.zh-CN.md)

通过延迟而非 CDN 区域解析域名。

这个工具从 Globalping 检索 DNS 记录，选择延迟最低的 IP 地址，并更新 hosts 文件。

## 快速开始

1.  从 [Releases](https://github.com/Zhousiru/opti-hosts/releases/latest) 下载。

    例如：

    ```bash
    sudo wget https://github.com/Zhousiru/opti-hosts/releases/download/latest/opti-hosts-x86_64-unknown-linux-gnu -O /usr/local/bin/opti-hosts
    sudo chmod +x /usr/local/bin/opti-hosts
    ```

2.  在你的 `/etc/hosts` 中添加指令：

    ```plaintext
    # ...

    # OPTI-HOSTS example.com [Beijing, HK * 2, AS6939]
    # 提示: example.com 的记录将在此处生成。

    # ...
    ```

3.  运行：

    ```bash
    sudo opti-hosts
    sudo opti-hosts --dry-run # 预览更改
    ```

4.  添加到你的 crontab（推荐）：

    ```bash
    0 */12 * * * opti-hosts
    ```

## Hosts 指令

```
# OPTI-HOSTS <域名> [<位置> * <限制>, <位置>, ...]
```

- **域名**

  你想要解析的域名。

- **位置**

  你想要从哪个地理位置或网络解析域名。

  参见 [Globalping Network](https://globalping.io/network)，或在 [Globalping](https://globalping.io/) 上尝试。

- **限制**

  用于特定位置的最大节点数。

  默认为 1。

## 用法

```bash
opti-hosts [选项]
```

```plaintext
选项:
      --dry-run      预览输出而不对 hosts 文件进行任何更改
      --file <FILE>  Hosts 文件路径 [默认: /etc/hosts]
  -h, --help         打印帮助
  -V, --version      打印版本
```
