package host

import "unsafe"

//go:wasmimport $root print
func hostPrint(ptr0, len0 int32)

func HostPrint(msg string) {
	hostPrint(*(*int32)(unsafe.Pointer(&msg)), int32(len(msg)))
}

// Export functions from host
var host Host = nil

func SetHost(i Host) {
	host = i
}

type Host interface {
	Run()
}

//go:export run
func HostRun() {
	host.Run()
}
