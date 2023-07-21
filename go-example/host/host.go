package host

// #include "host.h"
import "C"

// Import functions from host
func HostPrint(msg string) {
  var lower_msg C.host_string_t
  
  lower_msg.ptr = C.CString(msg)
  lower_msg.len = C.size_t(len(msg))
  defer C.host_string_free(&lower_msg)
  C.host_print(&lower_msg)
}

// Export functions from host
var host Host = nil
func SetHost(i Host) {
  host = i
}
type Host interface {
  Run() 
}
//export host_run
func HostRun() {
  host.Run()
  
}
