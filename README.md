# Opti Hosts

[[English]](./README.md) [[中文]](./README.zh-CN.md)

Resolve domains by latency, not CDN zones.

This tool retrieves DNS records from Globalping, selects IPs with lowest latency, and updates the hosts file.

## Quickstart

1.  Download from [Releases](https://github.com/Zhousiru/opti-hosts/releases/latest).

    For example:

    ```bash
    sudo wget https://github.com/Zhousiru/opti-hosts/releases/download/latest/opti-hosts-x86_64-unknown-linux-gnu -O /usr/local/bin/opti-hosts
    sudo chmod +x /usr/local/bin/opti-hosts
    ```

2.  Add directives in your `/etc/hosts`:

    ```plaintext
    # ...

    # OPTI-HOSTS example.com [Beijing, HK * 2, AS6939]
    # HINT: Record for example.com will be generated here.

    # ...
    ```

3.  Run:

    ```bash
    sudo opti-hosts
    sudo opti-hosts --dry-run # Preview changes
    ```

4.  Add to your crontab (recommend):

    ```bash
    0 */12 * * * opti-hosts
    ```

## Hosts Directive

```
# OPTI-HOSTS <Domain> [<Location> * <Limit>, <Location>, ...]
```

- **Domain**

  The domain you want to resolve.

- **Location**

  The geographic location or network from which you want to resolve the domain.

  See [Globalping Network](https://globalping.io/network), or try it on [Globalping](https://globalping.io/).

- **Limit**

  The maximum number of nodes to use for a particular location.

  Defaults to 1.

## Usage

```bash
opti-hosts [OPTIONS]
```

```plaintext
Options:
      --dry-run      Preview outputs without making any changes to hosts file
      --file <FILE>  Hosts file path [default: /etc/hosts]
  -h, --help         Print help
  -V, --version      Print version
```
