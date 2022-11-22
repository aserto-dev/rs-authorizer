SHELL 	   := $(shell which bash)

NO_COLOR   :=\033[0m
OK_COLOR   :=\033[32;01m
ERR_COLOR  :=\033[31;01m
WARN_COLOR :=\033[36;01m
ATTN_COLOR :=\033[33;01m

REPO       := "buf.build/aserto-dev/authorizer"
COMMIT     ?= $(shell git rev-parse --short HEAD 2>/dev/null)
DATE       ?= $(shell date "+%FT%T%z")
LATEST 	   := $(shell buf beta registry tag list ${REPO} --format json --reverse | jq -r '.results | .[0].name')

.PHONY: all
all: deps generate build

.PHONY: deps
deps:
	@echo -e "${ATTN_COLOR}==> $@ ${NO_COLOR}"
	@go install github.com/bufbuild/buf/cmd/buf@v1.9.0

.PHONY: generate
generate:
	@echo -e "${ATTN_COLOR}==> $@ VERSION=${LATEST} ${NO_COLOR}"
	@PATH=$HOME/.cargo/bin:${PATH} buf generate ${REPO}:${LATEST}

.PHONY: build
build:
	@echo -e "${ATTN_COLOR}==> $@ ${NO_COLOR}"
	@cargo build

.PHONY: clean
clean:
	@echo -e "${ATTN_COLOR}==> $@ ${NO_COLOR}"
	@cargo clean
