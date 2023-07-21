package main

import (
	host "go-example/host"
)

func init() {
	a := HostImpl{}
	host.SetHost(a)
}

type HostImpl struct{}

func (e HostImpl) Run() {
	host.HostPrint("Hello, world!")
}

func main() {}
