Based on the tutorial by Jon Gjengset: https://www.youtube.com/watch?v=bzja9fQWzdA&t=8s

Only tested on Mac


Once the application is running, you need to register a route in the route table, to communicate with the utun interface:

1. Check the created interface with ifconfig
2. Add the route

```shell
sudo route -n add -net 10.0.0.1 -interface utun5
```