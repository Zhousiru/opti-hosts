# Opti Hosts

Resolve domains by latency, not CDN zones.

> [!WARNING]
> Under development...

## How to Use

Edit your `/etc/hosts`:

```text
# ...

# OPTI-HOSTS example.com [Beijing, HK * 2, AS6939]
# HINT: Record for example.com will be generated here.

# ...
```

## Explanation

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
