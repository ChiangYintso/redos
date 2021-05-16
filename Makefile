KERNEL_BIN:=redos

run:
	@make -C user build
	@make -C os run KERNEL_BIN=$(KERNEL_BIN)

clean:
	@make -C user clean
	@make -C os clean

fmt:
	@cd os && cargo fmt
	@cd os/src/algorithm && cargo fmt
	@cd user && cargo fmt